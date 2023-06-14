use serde::{ de, Serialize, Deserialize, Deserializer };

//Map Klinedata to a struct
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KlineData {
    pub open_time: i64,
    #[serde(deserialize_with = "de_float_from_str")]
    pub open: f64,
    #[serde(deserialize_with = "de_float_from_str")]
    pub high: f64,
    #[serde(deserialize_with = "de_float_from_str")]
    pub low: f64,
    #[serde(deserialize_with = "de_float_from_str")]
    pub close: f64,
    #[serde(deserialize_with = "de_float_from_str")]
    pub volume: f64,
    pub close_time: i64,
    #[serde(deserialize_with = "de_float_from_str")]
    pub quote_asset_volume: f64,
    pub number_of_trades: usize,
    #[serde(deserialize_with = "de_float_from_str")]
    pub taker_buy_base_asset_volume: f64,
    #[serde(deserialize_with = "de_float_from_str")]
    pub taker_buy_quote_asset_volume: f64,
    #[serde(deserialize_with = "de_float_from_str")]
    pub ignore: f64,
}

pub fn de_float_from_str<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where D: Deserializer<'de>
{
    let str_val = String::deserialize(deserializer)?;
    Ok(str_val.parse::<f64>().map_err(de::Error::custom)?)
}
