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
    let result = binance::get_klines(client.clone(), "1m", "BTCUSDT", 500).await;

    let kline_data = match result {
        Some(kline_data) => kline_data,
        _ => { panic!("Failed to get kline data") }
    };
    println!("first result: {:?}", kline_data[0]);

    // Get Vec of f64 based on the close prices for the period. We reverse the order of the vec with rev() and then take(100) to take the 100 latest values
    let price_data: Vec<f64> = kline_data
        .iter()
        .rev()
        .take(100)
        .map(|x| x.close)
        .collect();
    let result = statistics::simple_moving_average(&price_data, 26);

    let sma_data = match result {
        Some(sma_data) => sma_data,
        _ => { panic!("Failed to get SMA data") }
    };
    println!("SMAs: {:?}", sma_data);

    let result = statistics::exponential_moving_average(&price_data, 26);

    let ema_data = match result {
        Some(data) => data,
        _ => { panic!("Failed Calculate EMA") }
    };
    println!("EMAs: {:?}", ema_data);
}
