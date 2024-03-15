use std::time::Duration;

use aws_sdk_s3::{presigning::PresigningConfig, types::ChecksumAlgorithm};

#[tokio::main]
async fn main() {
    let client = aws_sdk_s3::Client::new(&aws_config::load_from_env().await);

    // checksum for b"abc"
    let checksum = "ungWv48Bz+pBQUDeXa4iI7ADYaOWF3qctBD/YfIAFa0=";

    let presigned_request = client
        .put_object()
        .bucket("example-bucket")
        .key("foo")
        .checksum_algorithm(ChecksumAlgorithm::Sha256)
        .checksum_sha256(checksum)
        .presigned(PresigningConfig::expires_in(Duration::from_secs(360)).unwrap())
        .await
        .unwrap();

    dbg!(presigned_request.headers().collect::<Vec<_>>());

    assert!(presigned_request
        .headers()
        .any(|(k, v)| { k == "x-amz-content-sha256" && v == checksum }));
}
