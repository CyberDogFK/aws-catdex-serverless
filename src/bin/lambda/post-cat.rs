use aws_sdk_dynamodb as dynamodb;
use aws_sdk_dynamodb::model::AttributeValue;
use lambda_http::{http::StatusCode, run, service_fn, Body, Error, Request, Response, RequestPayloadExt};
use serde::Deserialize;

#[derive(Deserialize)]
struct RequestBody {
    name: String,
}

async fn function_handler(
    request: Request,
    client: &dynamodb::Client,
    table_name: &str,
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

    let dynamo_request = client
        .put_item()
        .table_name(table_name)
        .item("cat", AttributeValue::S(body.name.clone()));
    dynamo_request.send().await?;

    let resp = Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "text/html")
        .body(format!("Added cat {}", body.name).into())
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
    let client = dynamodb::Client::new(&config);
    let table_name = std::env::var("TABLE_NAME")?.to_string();

    run(service_fn(|request| {
        function_handler(request, &client, &table_name)
    }))
        .await
}