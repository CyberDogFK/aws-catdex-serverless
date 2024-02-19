use aws_sdk_dynamodb as dynamodb;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_s3 as s3;
use lambda_http::{
    http::StatusCode, run, service_fn, Body, Error, Request, RequestPayloadExt, Response,
};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use aws_sdk_s3::presigning::config::PresigningConfig;

#[derive(Deserialize)]
struct RequestBody {
    name: String,
}

#[derive(Serialize)]
struct ResponseBody {
    upload_url: String,
}

async fn function_handler(
    request: Request,
    dynamo_client: &dynamodb::Client,
    s3_client: &s3::Client,
    table_name: &str,
    bucket_name: &str,
) -> Result<Response<Body>, Error> {
    let body: RequestBody = match request.payload() {
        Ok(Some(body)) => body,
        _ => {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body("Invalid payload".into())
                .expect("Failed to render response"))
        }
    };

    let presigned_request = s3_client
        .put_object()
        .bucket(bucket_name)
        .key(&body.name)
        .presigned(PresigningConfig::expires_in(Duration::from_secs(60))?)
        .await?;

    let response_body = ResponseBody {
        upload_url: presigned_request.uri().to_string(),
    };

    let dynamo_request = dynamo_client
        .put_item()
        .table_name(table_name)
        .item("cat", AttributeValue::S(body.name.clone()));
    dynamo_request.send().await?;

    let resp = Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "text/html")
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Headers", "Content-Type")
        .header("Access-Control-Allow-Methods", "GET")
        .body(serde_json::to_string(&response_body)?.into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    let config = aws_config::load_from_env().await;
    let dynamo_client = dynamodb::Client::new(&config);
    let s3_client = s3::Client::new(&config);
    let table_name = std::env::var("TABLE_NAME")?.to_string();
    let bucket_name = std::env::var("BUCKET_NAME")?.to_string();

    run(service_fn(|request| {
        function_handler(
            request,
            &dynamo_client,
            &s3_client,
            &table_name,
            &bucket_name,
        )
    }))
    .await
}
