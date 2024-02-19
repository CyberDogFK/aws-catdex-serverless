use aws_sdk_s3 as s3;
use aws_sdk_s3::model::{BucketLocationConstraint, CreateBucketConfiguration};

#[tokio::main]
async fn main() {
    let config = aws_config::load_from_env()
        .await;
    let s3_client = s3::Client::new(&config);

    let constraint = BucketLocationConstraint::from("us-east-2");
    let cfg = CreateBucketConfiguration::builder()
        .location_constraint(constraint)
        .build();
    s3_client
        .create_bucket()
        .create_bucket_configuration(cfg)
        .bucket("catdex-frontend")
        .send()
        .await
        .unwrap();
}