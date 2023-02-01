//create a public function that will be called in the main function
pub fn quote_of_the_day(quote_type: &str) -> String {
    // create a switch statement to return the quote of the day
    let quote = match quote_type {
        "motivational" => {
            "The only peole who will know are the people who keep moving forward. - Eren Yeager"
        }
        "sad" => "We'll never amount to anything. Not you, not I. - Hawkwood the Deserter",
        "lie" => "You are awesome!",
        _ => "No quote of the day",
    };

    quote.to_string()
}
