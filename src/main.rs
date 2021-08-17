use serde::Deserialize;
use reqwest::Error;
// use std::env;
use colored::*;

#[derive(Deserialize, Debug)]
struct Ticker {
    ticker: TickerData,
}

#[derive(Deserialize, Debug)]
struct TickerData {
    market_id: String,
    last_price: Vec<String>,
    min_ask: Vec<String>,
    max_bid: Vec<String>,
    volume: Vec<String>,
    price_variation_24h: String,
    price_variation_7d: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut vec = Vec::new();
    let market_ids = ["btc-clp", "eth-clp", "bch-clp", "ltc-clp"];
    for ids in market_ids {
        let request_url = format!("https://www.buda.com/api/v2/markets/{}/ticker",ids);
        let ticker_data = reqwest::get(&request_url)
            .await?
            .json::<Ticker>()
            .await?;
        vec.push(ticker_data.ticker);
    }
    println!("{:<14} {:>10} {:>10} {:>10} {:>10}",
        "",
        vec[0].market_id.bold(),
        vec[1].market_id.bold(),
        vec[2].market_id.bold(),
        vec[3].market_id.bold(),
    );
    println!("{:<14} {:>10} {:>10} {:>10} {:>10}",
        "Última orden",
        vec[0].last_price[0].parse::<f32>().unwrap(),
        vec[1].last_price[0].parse::<f32>().unwrap(),
        vec[2].last_price[0].parse::<f32>().unwrap(),
        vec[3].last_price[0].parse::<f32>().unwrap(),
    );
    println!("{:<14} {:>10} {:>10} {:>10} {:>10}",
        "Min venta",
        vec[0].min_ask[0].parse::<f32>().unwrap(),
        vec[1].min_ask[0].parse::<f32>().unwrap(),
        vec[2].min_ask[0].parse::<f32>().unwrap(),
        vec[3].min_ask[0].parse::<f32>().unwrap(),
    );
    println!("{:<14} {:>10} {:>10} {:>10} {:>10}",
        "Max compra",
        vec[0].max_bid[0].parse::<f32>().unwrap(),
        vec[1].max_bid[0].parse::<f32>().unwrap(),
        vec[2].max_bid[0].parse::<f32>().unwrap(),
        vec[3].max_bid[0].parse::<f32>().unwrap(),
    );
    println!("{:<14} {:>8.2} % {:>8.2} % {:>8.2} % {:>8.2} %",
        "Variación 24h",
        vec[0].price_variation_24h.parse::<f32>().unwrap() * 100.0,
        vec[1].price_variation_24h.parse::<f32>().unwrap() * 100.0,
        vec[2].price_variation_24h.parse::<f32>().unwrap() * 100.0,
        vec[3].price_variation_24h.parse::<f32>().unwrap() * 100.0,
    );
    println!("{:<14} {:>8.2} % {:>8.2} % {:>8.2} % {:>8.2} %",
        "Variación 7d",
        vec[0].price_variation_7d.parse::<f32>().unwrap() * 100.0,
        vec[1].price_variation_7d.parse::<f32>().unwrap() * 100.0,
        vec[2].price_variation_7d.parse::<f32>().unwrap() * 100.0,
        vec[3].price_variation_7d.parse::<f32>().unwrap() * 100.0,
    );

    Ok(())
}
