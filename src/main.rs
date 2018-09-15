#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate reqwest;
extern crate separator;

use serde::{Deserialize, Deserializer};
use serde::de::Error;
use separator::Separatable;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct BinanceTicker {
    symbol: String,
    bid_price: String,
    ask_price: String,
    high_price: String,
    low_price: String
}

#[derive(Deserialize, Debug)]
struct CmcTickerResponse {
    content: CmcTicker
}

#[derive(Deserialize, Debug)]
struct CmcTicker {
    price_usd: String,
    percent_change_1h: String,
    percent_change_24h: String,
    percent_change_7d: String,
    rank: String,
    #[serde(rename="24h_volume_btc")]
    //#[serde(deserialize_with = "float_from_str")]
    last_24h_volume_btc: String,
    #[serde(rename="24h_volume_usd")]
    //#[serde(deserialize_with = "float_from_str")]
    last_24h_volume_usd: String
}

enum Decimal {
    Zero
}

fn main() {
//    let tick: Ticker = ticker("NANOBTC");

    // unwrap return the "Ok" part
    let binance_nano_ticker = binance_ticker("NANOBTC").unwrap();
    let cmc_nano_ticker = cmc_ticker("nano").unwrap();
    let cmc_btc_ticker = cmc_ticker("bitcoin").unwrap();

    println!("Hello {}", binance_nano_ticker.symbol);
    println!("buy: {}", binance_nano_ticker.bid_price);
    println!("sell: {}", binance_nano_ticker.ask_price);

//    print ('vol: %.0f BTC (%s) | color=#000000'% (float(result_cmc_nano[0]['24h_volume_btc']), locale.currency(float(result_cmc_nano[0]['24h_volume_usd']), grouping=True)))
//    print ('change-24h: %.1f%% | color=#000000'% float(result_cmc_nano[0]['percent_change_24h']))
    println!(
        "vol: {} BTC (${}) | color=#000000",
        thousands(cmc_nano_ticker.last_24h_volume_btc, Decimal::Zero),
        thousands(cmc_nano_ticker.last_24h_volume_usd, Decimal::Zero)
    );
}


fn cmc_ticker(symbol: &str) -> Result<CmcTicker, reqwest::Error> {
    let request_url = format!(
        "https://api.coinmarketcap.com/v1/ticker/{symbol}/?convert=BTC",
        symbol = symbol
    );

    println!("Query {}...", request_url);
    let mut response = reqwest::get(&request_url)?;

    let ticker: CmcTickerResponse = response.json()?;

    println!("{:?}", ticker);

    Ok(ticker.content)
}

fn binance_ticker(symbol: &str) -> Result<BinanceTicker, reqwest::Error> {
    let request_url = format!(
        "https://api.binance.com/api/v1/ticker/24hr?symbol={symbol}",
        symbol = symbol
    );

    println!("Query {}...", request_url);
    let mut response = reqwest::get(&request_url)?;

    let ticker: BinanceTicker = response.json()?;
    println!("{:?}", ticker);

    Ok(ticker)
}

/**
 * Cast float from strings
 * this function is called from annotations with
 * #[serde(deserialize_with = "float_from_str")]
 * @deprecated
 */
fn float_from_str<'de, D>(deserializer: D) -> Result<f64, D::Error> where D: Deserializer<'de> {
    let s = <String>::deserialize(deserializer)?;

    let casted: f64 = s.parse().unwrap();

    Ok(casted)
}

fn thousands(number: String, decimal: Decimal) -> String {

    let number: f64 = number.parse().unwrap();

    // Limit the number of decimals, this convert to string
    let number = match decimal {
        Decimal::Zero => format!("{:.0}", number),
        Decimal::Two => format!("{:.2}", number),
        Decimal::Eight => format!("{:.8}", number)
    };

    // To separate the thousands, separated_string needs a number, so we re-cast
    let number: f64 = number.parse().unwrap();

    number.separated_string()
}