use aws_sdk_s3  as s3;
use aws_sdk_s3::model::{CorsConfiguration, CorsRule, IndexDocument, WebsiteConfiguration};
use aws_sdk_s3::types::ByteStream;
use std::path::Path;

#[tokio::main]
async fn main() {
    let config = aws_config::load_from_env().await;
    let s3_client = s3::Client::new(&config);

    let body = ByteStream::from_path(
        Path::new("./client/dist/index.html")
    )
        .await
        .unwrap();
    s3_client
        .put_object()
        .body(body)
        .bucket("catdex-frontend")
        .key("index.html")
        .content_type("text/html")
        .send()
        .await
        .unwrap();

    let body = ByteStream::from_path(
        Path::new("./client/dist/add.html")
    )
        .await
        .unwrap();
    s3_client
        .put_object()
        .body(body)
        .bucket("catdex-frontend")
        .key("add.html")
        .content_type("text/html")
        .send()
        .await
        .unwrap();

    let body = ByteStream::from_path(
        Path::new("./client/dist/css/index.css")
    )
        .await
        .unwrap();
    s3_client
        .put_object()
        .body(body)
        .bucket("catdex-frontend")
        .key("css/index.css")
        .content_type("text/css")
        .send()
        .await
        .unwrap();

    let cors_rule_1 = CorsRule::builder()
        .allowed_headers("*")
        .allowed_methods("PUT")
        .allowed_methods("POST")
        .allowed_origins("http://*.amazonaws.com")
        .max_age_seconds(0)
        .build();

    let cors_rule_2 = CorsRule::builder()
        .allowed_headers("*")
        .allowed_methods("GET")
        .allowed_origins("*")
        .build();

    let cfg = CorsConfiguration::builder()
        .cors_rules(cors_rule_1)
        .cors_rules(cors_rule_2)
        .build();

    s3_client
        .put_bucket_cors()
        .bucket("catdex-frontend")
        .cors_configuration(cfg)
        .send()
        .await
        .unwrap();

    let cfg = WebsiteConfiguration::builder()
        .index_document(
            IndexDocument::builder()
                .suffix("index.html").build()
        )
        .build();
    s3_client
        .put_bucket_website()
        .bucket("catdex-frontend")
        .website_configuration(cfg)
        .send()
        .await
        .unwrap();

    s3_client
        .put_bucket_policy()
        .bucket("catdex-frontend")
        .policy(include_str!("../../bucket_policy.json"))
        .send()
        .await
        .unwrap();
}