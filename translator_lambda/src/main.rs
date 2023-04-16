use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use rust_bert::pipelines::translation::{Language, TranslationModelBuilder};

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {
    sentence: String,
}

#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: std::vec::Vec<String>,
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    // Extract some useful info from the request
    let sentence = event.payload.sentence;
    let model = TranslationModelBuilder::new()
        .with_source_languages(vec![Language::English])
        .with_target_languages(vec![Language::Spanish, Language::French, Language::Italian])
        .create_model()?;
    // Ask the user for a sentence to translate
    let output = model.translate(&[sentence], None, Language::French)?;
    // Prepare the response
    let resp = Response {
        req_id: event.context.request_id,
        msg: output,
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
