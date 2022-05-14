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
    content_type: &str,
) -> Result<PutObjectOutput, Box<dyn Error>> {
    let create_request = client
        .put_object()
        .bucket(bucket)
        .body(body)
        .key(key)
        .acl(aws_sdk_s3::model::ObjectCannedAcl::PublicRead)
        .content_type(content_type)
        .send()
        .await?;

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

    Ok(presigned_request)
}
