extern crate serde;
extern crate reqwest;

#[derive(Deserialize, Debug)]
pub struct CmcTickerResponse {
    pub content: CmcTicker
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct CmcTicker {
    pub price_usd: String,
    pub percent_change_1h: String,
    pub percent_change_24h: String,
    pub percent_change_7d: String,
    pub rank: String,
    #[serde(rename="24h_volume_btc")]
    //#[serde(deserialize_with = "float_from_str")]
    pub last_24h_volume_btc: String,
    #[serde(rename="24h_volume_usd")]
    //#[serde(deserialize_with = "float_from_str")]
    pub last_24h_volume_usd: String
}

impl CmcTicker {

    pub fn ticker(symbol: &str) -> Result<CmcTicker, reqwest::Error> {
        let request_url = format!(
            "https://api.coinmarketcap.com/v1/ticker/{symbol}/?convert=BTC",
            symbol = symbol
        );

        let response: CmcTickerResponse = reqwest::get(&request_url)?.json()?;

        Ok(response.content)
    }

}