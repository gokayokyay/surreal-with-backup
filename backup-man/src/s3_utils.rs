use awscreds::Credentials;
use awsregion::Region;
use s3::Bucket;
use tokio::io::AsyncRead;

use crate::config::{get_config, CONFIG_KEY};

pub async fn upload_file_stream<R>(path: String, reader: &mut R)
where
    R: AsyncRead + Unpin,
{
    let bucket_name = get_config(CONFIG_KEY::S3_BUCKET_NAME);
    if bucket_name == "" {
        tracing::warn!(
            "Bucket name isn't provided. Skipping S3. (Which is all this package does...)"
        );
        return;
    }

    let region = get_config(CONFIG_KEY::S3_REGION);
    if region == "" {
        tracing::warn!(
            "S3 region isn't provided. Skipping S3. (Which is all this package does...)"
        );
        return;
    }

    let endpoint = get_config(CONFIG_KEY::S3_ENDPOINT);
    if endpoint == "" {
        tracing::warn!(
            "S3 endpoint isn't provided. Skipping S3. (Which is all this package does...)"
        );
        return;
    }

    let access_key = get_config(CONFIG_KEY::S3_ACCESS_KEY);
    if access_key == "" {
        tracing::warn!(
            "S3 access key isn't provided. Skipping S3. (Which is all this package does...)"
        );
        return;
    }
    let secret_key = get_config(CONFIG_KEY::S3_SECRET_KEY);
    if secret_key == "" {
        tracing::warn!(
            "S3 secret key isn't provided. Skipping S3. (Which is all this package does...)"
        );
        return;
    }

    let session_token = std::env::var("S3_SESSION_TOKEN").ok();
    let session_token = session_token.as_ref().map(|x| &**x);
    let security_token = std::env::var("S3_SECURITY_TOKEN").ok();
    let security_token = security_token.as_ref().map(|x| &**x);

    let bucket = match Bucket::new(
        &bucket_name,
        Region::Custom { region, endpoint },
        Credentials::new(
            Some(&access_key),
            Some(&secret_key),
            security_token,
            session_token,
            None,
        )
        .unwrap(),
    ) {
        Ok(b) => b,
        Err(e) => {
            tracing::error!("{e}");
            eprintln!("Bucket obj creation failed. Normally its okay to retry but in this case your config is wrong. So panicking, please fix your config.");
            panic!();
        }
    };
    let bucket = bucket.with_path_style();
    let _result = bucket.put_object_stream(reader, path).await;
}
