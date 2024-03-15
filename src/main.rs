use std::time::Duration;

use aws_sdk_s3::{
    presigning::{PresignedRequest, PresigningConfig},
    primitives::ByteStream,
    types::ChecksumAlgorithm,
};

#[tokio::main]
async fn main() {
    let client = aws_sdk_s3::Client::new(&aws_config::load_from_env().await);

    let req1 =
        create_presigned_request(&client, "ungWv48Bz+pBQUDeXa4iI7ADYaOWF3qctBD/YfIAFa0=").await;
    let req2 =
        create_presigned_request(&client, "Ngi8oeROpsTSaOttsCJgJpiSwLQrhrvx53pvoWw8koI=").await;

    assert_ne!(
        req1.headers().collect::<Vec<_>>(),
        req2.headers().collect::<Vec<_>>()
    );
}

async fn create_presigned_request(client: &aws_sdk_s3::Client, checksum: &str) -> PresignedRequest {
    client
        .put_object()
        .bucket("example-bucket")
        .key("foo")
        .checksum_algorithm(ChecksumAlgorithm::Sha256)
        .checksum_sha256(checksum)
        .body(ByteStream::from_static(b""))
        .presigned(PresigningConfig::expires_in(Duration::from_secs(360)).unwrap())
        .await
        .unwrap()
}
