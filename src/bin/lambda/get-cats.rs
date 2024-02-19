use aws_sdk_dynamodb as dynamodb;
use lambda_http::{http::StatusCode, run, service_fn, Body, Error, Request, Response};
use serde::Serialize;

#[derive(Serialize)]
struct ResponseBody<'a> {
    cats: Vec<&'a String>,
}

async fn function_handler(
    _request: Request,
    client: &dynamodb::Client,
    table_name: &str,
) -> Result<Response<Body>, Error> {
    let scan_output = client.scan().table_name(table_name).send().await;

    let scan_output = scan_output?;
    let response_body = ResponseBody {
        cats: scan_output
            .items()
            .unwrap_or_default()
            .into_iter()
            .map(|val| val.get("cat").unwrap().as_s().unwrap())
            .collect(),
    };

    let resp = Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "text/html")
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Headers", "Content-Type")
        .header("Access-Control-Allow-Methods", "GET")
        .body(serde_json::to_string(&response_body).unwrap().into())
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
