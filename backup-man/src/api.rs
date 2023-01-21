use futures::{future::BoxFuture, TryStreamExt};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio_util::io::StreamReader;

use crate::{
    config::{get_config, CONFIG_KEY},
    s3_utils::upload_file_stream,
    utils::handle_retry,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
struct APIResponse {
    result: Value,
    status: String,
    time: String,
}

pub fn get_default_base_client_post(path: String) -> reqwest::RequestBuilder {
    let url = format!("{}{path}", get_config(CONFIG_KEY::DB_URL));
    let res = reqwest::Client::new()
        .post(url)
        .basic_auth(
            get_config(CONFIG_KEY::DB_USERNAME),
            get_config(CONFIG_KEY::DB_PASSWORD).into(),
        )
        .header("Accept", "application/json");
    return res;
}

pub fn get_default_base_client_get(path: String) -> reqwest::RequestBuilder {
    let url = format!("{}{path}", get_config(CONFIG_KEY::DB_URL));
    let res = reqwest::Client::new()
        .get(url)
        .basic_auth(
            get_config(CONFIG_KEY::DB_USERNAME),
            get_config(CONFIG_KEY::DB_PASSWORD).into(),
        )
        .header("Accept", "application/json");
    return res;
}

pub async fn check_credentials(url: String, username: String, password: String) {
    check_credentials_inner(url, username, password, 0).await;
}

fn check_credentials_inner(
    url: String,
    username: String,
    password: String,
    retry_count: u8,
) -> BoxFuture<'static, ()> {
    Box::pin(async move {
        handle_retry(retry_count).await;
        let credential_error_msg =
            "Error database credentials or endpoint are not correct. Can't backup. Retrying...";
        let failed_req_error_msg = "Credential check request was not successful. Retrying...";
        let res = match reqwest::Client::new()
            .post(&url)
            .basic_auth(username.clone(), password.clone().into())
            .header("Accept", "application/json")
            .body("INFO FOR KV;")
            .send()
            .await
        {
            Ok(r) => r,
            Err(e) => {
                tracing::error!("{failed_req_error_msg}");
                tracing::error!("{e}");
                return check_credentials_inner(url, username, password, retry_count + 1).await;
            }
        };
        let _res: Vec<APIResponse> = match res.json().await {
            Ok(r) => r,
            Err(e) => {
                tracing::error!("{credential_error_msg}");
                tracing::error!("{e}");
                return check_credentials_inner(url, username, password, retry_count + 1).await;
            }
        };
        tracing::info!("Credentials check has passed!");
        return;
    })
}

#[tracing::instrument(name = "Fetching namespaces")]
pub fn get_namespaces(retry_count: u8) -> BoxFuture<'static, Vec<String>> {
    if retry_count == 0 {
        tracing::info!("Getting available namespaces.");
    }
    Box::pin(async move {
        let error = "Getting namespace info failed. Will try again.";
        handle_retry(retry_count).await;
        let client = get_default_base_client_post("/sql".into());
        let res: Vec<APIResponse> = match client.body("INFO FOR KV;").send().await {
            Ok(res) => match res.json().await {
                Ok(o) => o,
                Err(e) => {
                    tracing::error!("{error}");
                    tracing::error!("{e}");
                    return get_namespaces(retry_count + 1).await;
                }
            },
            Err(e) => {
                tracing::error!("{error}");
                tracing::error!("{e}");
                return get_namespaces(retry_count + 1).await;
            }
        };
        if res.len() < 1 {
            tracing::info!("SurrealDB is empty, no need to backup.");
            return vec![];
        }
        let namespaces = res[0].result.as_object().unwrap()["ns"]
            .as_object()
            .unwrap()
            .keys()
            .map(|k| {
                return k.to_owned();
            })
            .collect::<Vec<String>>();
        return namespaces;
    })
}

#[tracing::instrument(name = "Fetching databases")]
pub fn get_databases(namespace: String, retry_count: u8) -> BoxFuture<'static, Vec<String>> {
    if retry_count == 0 {
        tracing::info!("Getting available databases for namespace {namespace}.");
    }
    Box::pin(async move {
        let error = "Getting database info failed. Will try again.";
        handle_retry(retry_count).await;
        let client = get_default_base_client_post("/sql".into());
        let res: Vec<APIResponse> = match client
            .header("NS", namespace.clone())
            .body("INFO FOR NS;")
            .send()
            .await
        {
            Ok(res) => match res.json().await {
                Ok(o) => o,
                Err(e) => {
                    tracing::error!("{error}");
                    tracing::error!("{e}");
                    return get_databases(namespace, retry_count + 1).await;
                }
            },
            Err(e) => {
                tracing::error!("{error}");
                tracing::error!("{e}");
                return get_databases(namespace, retry_count + 1).await;
            }
        };

        if res.len() < 1 {
            tracing::info!("Namespace is empty, no need to backup.");
            return vec![];
        }
        let databases = res[0].result.as_object().unwrap()["db"]
            .as_object()
            .unwrap()
            .keys()
            .map(|k| {
                return k.to_owned();
            })
            .collect::<Vec<String>>();

        return databases;
    })
}

pub fn export_namespace_database(
    namespace: String,
    database: String,
    retry_count: u8,
) -> BoxFuture<'static, ()> {
    if retry_count == 0 {
        tracing::info!("Exporting NS:{namespace} DB:{database}.");
    }
    Box::pin(async move {
        let error = "Exporting database failed. Will try again.";
        handle_retry(retry_count).await;
        let client = get_default_base_client_get("/export".into());
        let stream = match client
            .header("NS", namespace.clone())
            .header("DB", database.clone())
            .send()
            .await
        {
            Ok(res) => res.bytes_stream().map_err(convert_reqwest_err),
            Err(e) => {
                tracing::error!("{error}");
                tracing::error!("{e}");
                return export_namespace_database(namespace, database, retry_count + 1).await;
            }
        };
        let pretty_date = chrono::Utc::now()
            .naive_utc()
            .format("%Y_%m_%d %H__%M__%S")
            .to_string();
        let stream = stream;
        let mut reader = StreamReader::new(stream);

        let path = format!("{namespace}_{database}_{pretty_date}.sql");
        upload_file_stream(path, &mut reader).await;
    })
}

fn convert_reqwest_err(err: reqwest::Error) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, err.to_string())
}
