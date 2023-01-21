use std::{collections::HashMap, env};

use strum::{Display, EnumString};
use tracing::{info, warn};

#[allow(non_camel_case_types)]
#[derive(Eq, PartialEq, Hash, EnumString, Display)]
pub enum CONFIG_KEY {
    DB_USERNAME,
    DB_PASSWORD,
    DB_URL,
    S3_ACCESS_KEY,
    S3_SECRET_KEY,
    S3_SECURITY_TOKEN,
    S3_SESSION_TOKEN,
    S3_BUCKET_NAME,
    S3_REGION,
    S3_ENDPOINT,
    PERIOD,
}

pub fn check_env() {
    let keys = vec![
        CONFIG_KEY::DB_USERNAME,
        CONFIG_KEY::DB_PASSWORD,
        CONFIG_KEY::DB_URL,
        CONFIG_KEY::S3_ACCESS_KEY,
        CONFIG_KEY::S3_SECRET_KEY,
        CONFIG_KEY::S3_SECURITY_TOKEN,
        CONFIG_KEY::S3_SESSION_TOKEN,
        CONFIG_KEY::S3_BUCKET_NAME,
        CONFIG_KEY::S3_REGION,
        CONFIG_KEY::S3_ENDPOINT,
        CONFIG_KEY::PERIOD,
    ];

    for key in keys {
        let key = key.to_string();
        if let Ok(val) = env::var(&key) {
            info!("\"{val}\" env found!");
        } else {
            warn!(
                "Environment variable: \"{key}\" not found! Will be using default value instead."
            );
        }
    }
}

pub fn get_config(key: CONFIG_KEY) -> String {
    let default_config = HashMap::from([
        (CONFIG_KEY::DB_USERNAME, "root"),
        (CONFIG_KEY::DB_PASSWORD, "root"),
        (CONFIG_KEY::DB_URL, "http://localhost:8000"),
        (CONFIG_KEY::S3_ACCESS_KEY, ""),
        (CONFIG_KEY::S3_SECRET_KEY, ""),
        (CONFIG_KEY::S3_SECURITY_TOKEN, ""),
        (CONFIG_KEY::S3_SESSION_TOKEN, ""),
        (CONFIG_KEY::S3_BUCKET_NAME, ""),
        (CONFIG_KEY::S3_REGION, ""),
        (CONFIG_KEY::S3_ENDPOINT, ""),
        (CONFIG_KEY::PERIOD, "daily"),
    ]);
    if let Ok(val) = env::var(key.to_string()) {
        return val;
    }
    return default_config[&key].to_string();
}
