mod api;
mod config;
mod s3_utils;
mod utils;

use api::{check_credentials, export_namespace_database, get_databases, get_namespaces};
use config::{check_env, get_config, CONFIG_KEY};
use daemonize_me::Daemon;
use std::fs::File;
use std::process::exit;
use tracing::{metadata::LevelFilter, Level};
use tracing_subscriber::{
    fmt::{layer, writer::MakeWriterExt},
    prelude::__tracing_subscriber_SubscriberExt,
    Layer,
};
use utils::get_seconds;

fn post_fork_child(ppid: i32, cpid: i32) {
    println!("Parent pid: {}, Child pid {}", ppid, cpid);
    println!("This hook is called in the child");
    // Child hook must return
    return;
}

fn main() {
    let file_appender =
        tracing_appender::rolling::daily(".", "backupman.log").with_max_level(Level::INFO);
    let stdout_log = layer()
        .with_filter(LevelFilter::INFO)
        .with_filter(LevelFilter::ERROR)
        .with_filter(LevelFilter::WARN);
    let debug_log = layer().with_writer(file_appender);
    let registry = tracing_subscriber::registry()
        .with(layer().with_filter(LevelFilter::INFO))
        .with(layer().with_filter(LevelFilter::ERROR))
        .with(layer().with_filter(LevelFilter::WARN))
        .with(debug_log.and_then(stdout_log));
    tracing::subscriber::set_global_default(registry).expect("setting default subscriber failed");

    check_env();

    let stdout = File::create("trace.log").unwrap();
    let stderr = File::create("err.log").unwrap();
    let daemon = Daemon::new()
        .pid_file("backman.pid", Some(false))
        .umask(0o000)
        .work_dir(".")
        .setup_post_fork_child_hook(post_fork_child)
        .stdout(stdout)
        .stderr(stderr)
        .start();

    match daemon {
        Ok(_) => println!("Daemonized with success"),
        Err(e) => {
            eprintln!("Error, {}", e);
            exit(-1);
        }
    }

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async move {
        check_credentials(
            format!("{}/sql", get_config(CONFIG_KEY::DB_URL)),
            get_config(CONFIG_KEY::DB_USERNAME),
            get_config(CONFIG_KEY::DB_PASSWORD),
        )
        .await;
        loop {
            let namespaces = get_namespaces(0).await;
            for namespace in namespaces {
                let dbs = get_databases(namespace.clone(), 0).await;
                for db in dbs {
                    let ns = namespace.clone();
                    tokio::spawn(async move { export_namespace_database(ns, db, 0).await });
                }
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(get_seconds(get_config(
                CONFIG_KEY::PERIOD,
            ))))
            .await;
        }
    });
}
