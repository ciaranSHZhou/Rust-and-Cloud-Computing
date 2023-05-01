

fn main() {
    let analyzer = vader_sentiment::SentimentIntensityAnalyzer::new();
    println!("{:#?}", analyzer.polarity_scores("VADER is smart, handsome, and funny."));
    println!("{:#?}", analyzer.polarity_scores("VADER is VERY SMART, handsome, and FUNNY."));
}