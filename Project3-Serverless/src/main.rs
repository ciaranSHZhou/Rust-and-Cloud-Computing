use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use rust_bert::pipelines::sentiment::SentimentModel;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {
    sentence: String,
}

#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    // Extract some useful info from the request
    let sentence = event.payload.sentence;
    //Create an array and put sentence to it
    let sentence: [&str; 1] = [sentence.as_str()];
    let sentiment_model = SentimentModel::new(Default::default())?;
    
    let output = sentiment_model.predict(sentence);// Prepare the response
    // define output_string and append each sentiment as string to it
    let mut output_string = String::new();
    for sentiment in output {
        output_string.push_str(&format!("{:?}", sentiment));
    }

 
    let resp = Response {
        req_id: event.context.request_id,
        msg: output_string,
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
