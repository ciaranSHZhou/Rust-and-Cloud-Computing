use lambda_runtime::{run, service_fn, Error, LambdaEvent};

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {
    theme: String,
}

#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    // Extract some useful info from the request
    let theme = event.payload.theme;
    let quote = match theme.as_str() {
        "motivational" => {
            "The only peole who will know are the people who keep moving forward. - Eren Yeager"
        }
        "sad" => "We'll never amount to anything. Not you, not I. - Hawkwood the Deserter",
        "lie" => "You are awesome! - Who",
        _ => "No quote of the day for you",
    };
    // Prepare the response
    let resp = Response {
        req_id: event.context.request_id,
        msg: format!("Theme {}: {}", theme, quote),
    };

    // Return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
