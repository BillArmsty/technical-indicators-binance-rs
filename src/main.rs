mod statistics;
mod models;
mod binance;

#[cfg(test)]
mod test_statistics;
mod utils;

//Tokio provides an attribute macro so that you can make your main function async
#[tokio::main]
async fn main() {
    let client = utils::get_client();
    let result = binance::get_klines(client.clone(), "1d", "BTCUSDT", 500).await;

    let kline_data = match result {
        Some(kline_data) => kline_data,
        _ => { panic!("Failed to get kline data") }
    };
    println!("first result: {:?}", kline_data[0]);
}
