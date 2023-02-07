use serde_json::Value;
use std::error::Error;

pub fn get_stock_price(name: &str) -> Result<f64, Box<dyn Error>> {
    let url = format!(
        "https://query1.finance.yahoo.com/v7/finance/quote?symbols={}",
        name
    );
    let resp = reqwest::blocking::get(url)?.text()?;
    let v: Value = serde_json::from_str(&resp)?;
    let price = v["quoteResponse"]["result"][0]["regularMarketPrice"]
        .as_f64()
        .unwrap();
    Ok(price)
}
