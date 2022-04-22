use std::{error::Error, time::Duration};

use aws_sdk_s3::{
    output::PutObjectOutput,
    presigning::{config::PresigningConfig, request::PresignedRequest},
    types::ByteStream,
    Client,
};

pub async fn create_object(
    client: &Client,
    bucket: &str,
    body: ByteStream,
    key: &str,
) -> Result<PutObjectOutput, Box<dyn Error>> {
    let create_request = client
        .put_object()
        .bucket(bucket)
        .body(body)
        .key(key)
        .send()
        .await?;

    println!("Create Request: {:?}", create_request);

    Ok(create_request)
}

pub async fn get_object(
    client: &Client,
    bucket: &str,
    object: &str,
    expires_in: u64,
) -> Result<PresignedRequest, Box<dyn Error>> {
    let expires_in = Duration::from_secs(expires_in);
    let presigned_request = client
        .get_object()
        .bucket(bucket)
        .key(object)
        .presigned(PresigningConfig::expires_in(expires_in)?)
        .await?;

    println!("Object URI: {}", presigned_request.uri());

    Ok(presigned_request)
}
