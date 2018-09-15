#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate reqwest;
extern crate separator;

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

fn main() {

    // unwrap return the "Ok" part
    let binance_nano_ticker = binance_ticker("NANOBTC").unwrap();
    let cmc_nano_ticker = cmc_ticker("nano").unwrap();
    let cmc_btc_ticker = cmc_ticker("bitcoin").unwrap();

    println!("buy: {}", binance_nano_ticker.bid_price);
    println!("sell: {}", binance_nano_ticker.ask_price);

    println!(
        "vol: {} BTC (${}) | color=#000000",
        thousands(cmc_nano_ticker.last_24h_volume_btc, 0),
        thousands(cmc_nano_ticker.last_24h_volume_usd, 2)
    );

    println!("change-24h: {}% | color=#000000", cmc_nano_ticker.percent_change_24h);

//    let high_usd: f64 = cmc_btc_ticker.price_usd;
//    let low_usd: f64 = number.parse().unwrap();
//    let f = 3.99_f64;

    println!("{:?}", f);
//    high_usd = float(result_cmc_btc[0]['price_usd']) * float(result_binance['highPrice'])
//    low_usd = float(result_cmc_btc[0]['price_usd']) * float(result_binance['lowPrice'])
//    print ('high:   %.8f (%s) | color=#000000'% (float(result_binance['highPrice']), locale.currency(high_usd, grouping=True)))
//    print ('low:    %.8f (%s) | color=#000000'% (float(result_binance['lowPrice']), locale.currency(low_usd, grouping=True)))

//    println!(
//        "high:   %.8f (%s) | color=#000000",
//        thousands(cmc_nano_ticker.last_24h_volume_btc, Decimal::Zero),
//        thousands(cmc_nano_ticker.last_24h_volume_usd, Decimal::Zero)
//    );
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

fn thousands(number: String, decimal: usize) -> String {

    let number: f64 = number.parse().unwrap();

    // Limit the number of decimals, this convert to string
    let number = format!("{:.*}", decimal, number);

    // To separate the thousands, separated_string needs a number, so we re-cast
    let number: f64 = number.parse().unwrap();

    number.separated_string()
}