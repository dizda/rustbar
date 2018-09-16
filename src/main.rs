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
    last_price: String,
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
    let img_up = "iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAABmJLR0QAyQACAALwzISXAAAACXBIWXMAAAsTAAALEwEAmpwYAAAAB3RJTUUH4AQHACkSBTjB+AAAALNJREFUOMvVk70NAjEMhb87WYiGBZAQU7ABNSVSWpZgEEagsJDoKBELUCEKFuBuCKTw0xyQC0lICe5i+/k9/wT+3opUUJQhcAUqa8I5ZQT4tANwioGTCkQZA9vmOQE2oUJFhL0DXBz33RpKUfCLfLTQJMx9IlEWuQr6QB3prGtNS1lwiMvEYo7ekNsKRBkB+y+rH1hDFVOwy7ids+gbVzrsM6CXeYDTF85xroB1ZoHb73ymB5RhJkpZTihGAAAAAElFTkSuQmCC";
    let img_down = "iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAABmJLR0QABACnAADQ9FZaAAAACXBIWXMAAAsTAAALEwEAmpwYAAAAB3RJTUUH4AQHACQ1FZwK3gAAAMRJREFUOMvNkjEKAjEQRZ+jKNjYKh5AbzCdjVcQj+BFPIKlp7EMeAJrUbASQVCEr80uG9cNbqe/Cgn/5WUI/DqNfBHM+kCzbs+lPUAr2pwBq5qABbB+M8gszkDvS/kOdAG5VBgEM4ApsP0CGLukjxlEoA0wSZR3Lo0qhxhZDIBDAmDA0wsBLD51CZeOwLKivHbprZx6AkAHuEXbD5fawYwywMqAzOKeDTTPvKqcTGZBMLsGs0utn5gADYEHcKp9e9ni//MCDtNCE3qjsIwAAAAASUVORK5CYII=";
    let percent_change_1h: f32 = cmc_nano_ticker.percent_change_1h.parse().unwrap();
    let img: &str;

    if percent_change_1h > 0. {
        img = img_up;
    } else {
        img = img_down;
    }

    println!(
        "{} (${}) | image={} color=#000000",
        thousands(&binance_nano_ticker.last_price, 8),
        thousands(&cmc_nano_ticker.price_usd, 2),
        img
    );


//    def flow():
//    if result_cmc_nano[0]['percent_change_1h'] > '0':
//        print (' %.8f ($%.2f) | image=iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAABmJLR0QAyQACAALwzISXAAAACXBIWXMAAAsTAAALEwEAmpwYAAAAB3RJTUUH4AQHACkSBTjB+AAAALNJREFUOMvVk70NAjEMhb87WYiGBZAQU7ABNSVSWpZgEEagsJDoKBELUCEKFuBuCKTw0xyQC0lICe5i+/k9/wT+3opUUJQhcAUqa8I5ZQT4tANwioGTCkQZA9vmOQE2oUJFhL0DXBz33RpKUfCLfLTQJMx9IlEWuQr6QB3prGtNS1lwiMvEYo7ekNsKRBkB+y+rH1hDFVOwy7ids+gbVzrsM6CXeYDTF85xroB1ZoHb73ymB5RhJkpZTihGAAAAAElFTkSuQmCC color=#000000'% (float(result_binance['lastPrice']), float(result_cmc_nano[0]['price_usd'])) )
//    else:
//    print (' %.8f ($%.2f) | image=iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAABmJLR0QABACnAADQ9FZaAAAACXBIWXMAAAsTAAALEwEAmpwYAAAAB3RJTUUH4AQHACQ1FZwK3gAAAMRJREFUOMvNkjEKAjEQRZ+jKNjYKh5AbzCdjVcQj+BFPIKlp7EMeAJrUbASQVCEr80uG9cNbqe/Cgn/5WUI/DqNfBHM+kCzbs+lPUAr2pwBq5qABbB+M8gszkDvS/kOdAG5VBgEM4ApsP0CGLukjxlEoA0wSZR3Lo0qhxhZDIBDAmDA0wsBLD51CZeOwLKivHbprZx6AkAHuEXbD5fawYwywMqAzOKeDTTPvKqcTGZBMLsGs0utn5gADYEHcKp9e9ni//MCDtNCE3qjsIwAAAAASUVORK5CYII= color=#000000'% (float(result_binance['lastPrice']), float(result_cmc_nano[0]['price_usd'])) )
//
//    flow()

    println!("---");

    println!("buy: {} | color=green", binance_nano_ticker.bid_price);
    println!("sell: {} | color=red", binance_nano_ticker.ask_price);

    println!(
        "vol: {} BTC (${}) | color=#000000",
        thousands(&cmc_nano_ticker.last_24h_volume_btc, 0),
        thousands(&cmc_nano_ticker.last_24h_volume_usd, 2)
    );

    println!("change-24h: {}% | color=#000000", cmc_nano_ticker.percent_change_24h);

    println!(
        "high: {} (${}) | color=#000000",
        binance_nano_ticker.high_price,
        thousands(&cmc_btc_ticker.price_usd.multiply(&binance_nano_ticker.high_price), 2)
    );
    println!(
        "low: {} (${}) | color=#000000",
        binance_nano_ticker.low_price,
        thousands(&cmc_btc_ticker.price_usd.multiply(&binance_nano_ticker.low_price), 2)
    );

    println!("price: ${} | color=#000000", thousands(&cmc_nano_ticker.price_usd, 2));
    println!("rank: #{} | color=#000000", cmc_nano_ticker.rank);
}

pub trait Testt {
    fn multiply(&self, other: &String) -> String;
}

impl Testt for String {
    fn multiply(&self, right: &String) -> String {
        let left: f64 = self.parse().unwrap();
        let right: f64 = right.parse().unwrap();

        (left * right).to_string()
    }
}


fn cmc_ticker(symbol: &str) -> Result<CmcTicker, reqwest::Error> {
    let request_url = format!(
        "https://api.coinmarketcap.com/v1/ticker/{symbol}/?convert=BTC",
        symbol = symbol
    );

//    println!("Query {}...", request_url);
    let mut response = reqwest::get(&request_url)?;

    let ticker: CmcTickerResponse = response.json()?;

//    println!("{:?}", ticker);

    Ok(ticker.content)
}

fn binance_ticker(symbol: &str) -> Result<BinanceTicker, reqwest::Error> {
    let request_url = format!(
        "https://api.binance.com/api/v1/ticker/24hr?symbol={symbol}",
        symbol = symbol
    );

//    println!("Query {}...", request_url);
    let mut response = reqwest::get(&request_url)?;

    let ticker: BinanceTicker = response.json()?;
//    println!("{:?}", ticker);

    Ok(ticker)
}

fn thousands(number: &String, decimal: usize) -> String {

    let number: f64 = number.parse().unwrap();

    // Limit the number of decimals, this convert to string
    let number = format!("{:.*}", decimal, number);

    // To separate the thousands, separated_string needs a number, so we re-cast
    let number: f64 = number.parse().unwrap();

    number.separated_string()
}