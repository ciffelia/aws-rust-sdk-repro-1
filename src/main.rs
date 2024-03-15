use std::time::Duration;

use aws_sdk_s3::{
    presigning::{PresignedRequest, PresigningConfig},
    primitives::ByteStream,
    types::ChecksumAlgorithm,
};
use base64::Engine;
use sha2::Digest;

#[tokio::main]
async fn main() {
    let client = aws_sdk_s3::Client::new(&aws_config::load_from_env().await);

    let req1 = create_presigned_request(&client, "abc").await;
    let req2 = create_presigned_request(&client, "xyz").await;

    assert_ne!(
        req1.headers().collect::<Vec<_>>(),
        req2.headers().collect::<Vec<_>>()
    );
}

async fn create_presigned_request(
    client: &aws_sdk_s3::Client,
    body: impl AsRef<[u8]>,
) -> PresignedRequest {
    let checksum = base64::engine::general_purpose::STANDARD.encode(sha2::Sha256::digest(&body));
    println!("body: {:?}, checksum: {}", body.as_ref(), checksum);

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
