extern crate serde;
extern crate reqwest;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BinanceTicker {
    pub symbol: String,
    pub last_price: String,
    pub bid_price: String,
    pub ask_price: String,
    pub high_price: String,
    pub low_price: String
}

impl BinanceTicker {

    pub fn ticker(symbol: &str) -> Result<BinanceTicker, reqwest::Error> {
        let request_url = format!(
            "https://api.binance.com/api/v1/ticker/24hr?symbol={symbol}",
            symbol = symbol
        );

        let response = reqwest::get(&request_url)
            .expect("Failed to connect to Binance.")
            .json()?
        ;

        Ok(response)
    }

}