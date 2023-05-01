

fn main() {
    let analyzer = vader_sentiment::SentimentIntensityAnalyzer::new();
    let output = analyzer.polarity_scores("VADER is smart, handsome, and funny.");
    let mut output_string = String::new();
    for (key, value) in output.iter() {
        output_string.push_str(key);
        output_string.push(':');
        output_string.push_str(&value.to_string());
        output_string.push(',');
    }
    output_string.pop();
    println!("{}", output_string);
}