#[derive(Deserialize, Debug)]
pub struct CmcTickerResponse {
    pub content: CmcTicker
}

#[derive(Deserialize, Debug)]
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