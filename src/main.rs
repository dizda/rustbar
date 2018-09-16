#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate reqwest;
extern crate separator;

use separator::Separatable;

// internal files
pub mod math;
pub mod ticker;

use math::Math;
use ticker::binance::BinanceTicker;
use ticker::cmc::*;


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
        binance_nano_ticker.last_price,
        thousands(&cmc_nano_ticker.price_usd, 2),
        img
    );

    println!("---");

    println!("NANO | image=iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAMAAAAoLQ9TAAAABGdBTUEAALGPC/xhBQAAAAFzUkdCAK7OHOkAAABRUExURUxpcVSJykmQ4kaS3lKLzWyUxESF2V+W0FKV7lGJyU6O4EmR3k6P3lSU6EyP3kmT30qP4keU3kCT5kWS4UiR4VKL4EOT40OQ6ESS5UWQ5kiQ4v2G/RwAAAAKdFJOUwDr///+EP4F/tfCPIxYAAAAQ0lEQVQY02NgoBngkoAy+ATBFIukFITPzs0LJDlEGBkYmJiAXGFxsDAHMztEgJNZFMQVgGpnFWMD0/w8UAEhLto5GgDaMgGRTwkAAgAAAABJRU5ErkJggg==");
    println!("buy: {} | color=green", binance_nano_ticker.bid_price);
    println!("sell: {} | color=red", binance_nano_ticker.ask_price);

    println!(
        "vol: {} BTC (${}) | color=#000000",
        thousands(&cmc_nano_ticker.last_24h_volume_btc, 0),
        thousands(&cmc_nano_ticker.last_24h_volume_usd, 0)
    );

    println!("change-24h: {}% | color=#000000", cmc_nano_ticker.percent_change_24h);

    println!(
        "high: {} (${}) | color=#000000",
        binance_nano_ticker.high_price,
        thousands(&cmc_btc_ticker.price_usd.multiply(&binance_nano_ticker.high_price, 2), 2)
    );
    println!(
        "low: {} (${}) | color=#000000",
        binance_nano_ticker.low_price,
        thousands(&cmc_btc_ticker.price_usd.multiply(&binance_nano_ticker.low_price, 2), 2)
    );

    println!("price: ${} | color=#000000", thousands(&cmc_nano_ticker.price_usd, 2));
    println!("rank: #{} | color=#000000", cmc_nano_ticker.rank);

    println!("---");


    println!("NANO daily-trading | image=iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAMAAAAoLQ9TAAAABGdBTUEAALGPC/xhBQAAAAFzUkdCAK7OHOkAAABRUExURUxpcVSJykmQ4kaS3lKLzWyUxESF2V+W0FKV7lGJyU6O4EmR3k6P3lSU6EyP3kmT30qP4keU3kCT5kWS4UiR4VKL4EOT40OQ6ESS5UWQ5kiQ4v2G/RwAAAAKdFJOUwDr///+EP4F/tfCPIxYAAAAQ0lEQVQY02NgoBngkoAy+ATBFIukFITPzs0LJDlEGBkYmJiAXGFxsDAHMztEgJNZFMQVgGpnFWMD0/w8UAEhLto5GgDaMgGRTwkAAgAAAABJRU5ErkJggg==");

    let spread_24h_btc = binance_nano_ticker.high_price.sub(&binance_nano_ticker.low_price, 8);
    let spread_24h_usd = spread_24h_btc.multiply(&cmc_btc_ticker.price_usd, 2);

    // gain if we swingtrade with 50k NANO
    let possible_gain_btc = spread_24h_btc.multiply(&50000_f64.to_string(), 2);
    let possible_gain_usd = possible_gain_btc.multiply(&cmc_btc_ticker.price_usd, 8);

    println!(
        "spread: {} (${}) | color=#000000",
        spread_24h_btc,
        thousands(&spread_24h_usd, 2)
    );

    println!(
        "possible gain: {} BTC (${}) | color=#000000",
        possible_gain_btc,
        thousands(&possible_gain_usd, 2)
    );

    println!("---");

    println!("Bitcoin | image=iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAC3klEQVR4nGWTTWicVRSGn3Pv/b75sWkSaTQmNSWgiHZjS6DGFCd2pRTcaDcRFy5MaNq6UXCnuNdFF7ZaEUFUsCgIWkTB2lmISN0IdiH+1AaZSir9SSeZmfvde46LQVRcHDiL8/DCeXkEwAwRwQDih48ueO+XVa2l2aYAxEuncHKuyvnN8onPvv43I38vF99u1XeONF4RZMUXEiwqKSuYEooSKYSccjKzU+Fm/zl5ut03Q8ROH/JMrBfVleJMsa08MLg+MEwVcOKCEOpY76ohTvE1VxsrperGs8VEdZArt1UC0H938URtvHZ4cLUfRSgJdYg3kcm9uJmHscvnIfXJnW/B12NtvCzjjXiitvTVEYnv798H7htTU8N5qi7uzocIe1dBPDa4jtRGSd8dR3/7EpoTJpZVnDjQeafZjhXBRHMCrSA00LU2utZGxmYBgcat+PuXkTvmIHZFVRkydsyhqaWxAlOHZTDFqk2oj0PqET8+RPzgEdyO3RQLL4JGsOw0VqDVYkDzZKUgMCxSDClHcJNz4MshlPqQ+uj696AJMZUqKWY26UyTDJMzAGytI+UIsn2a/Ovn2GADv3sJLJMvvAdmw1sdjhPLfxSimKlRbeKm5/F7VkAc+Yd3qD49SvzkKQgN/H1LUG1hqlaI4tDLAdNzLvAkfVPywMv0PG7nfkAID7yAv+sx3O17QBx27WcwBc3qSvPW17bEt+7dh9mwRgnOuh3x9zxOmHsWi11kbBa79gv25wXS+ePDHziXxeEq5UEB2Hr97tcao361dyNHUq/0My0ITfxMi/zjR+SLXyC17RCaGBIbY6HsbeSTzZWfVsVO49ncVcQYzpTb5EBvw4zUV0ydNHcIqYcZBiimrjHqJHbtbFmmg9xyqfpHppd21aem7VVnLIfgQ06QqgrEEbzgCyElS8Abv3d4fvblS0OZ/qfzyZkF7/WZlFnUzBQY3ktHHG1Td6o8vPYfnf8CyHGVDNs26GQAAAAASUVORK5CYII=");

    println!("price:  ${} | color=#000000", thousands(&cmc_btc_ticker.price_usd, 2));
    println!("change 1h:  {}% | color=#000000", cmc_btc_ticker.percent_change_1h);
    println!("change 24h:  {}% | color=#000000", cmc_btc_ticker.percent_change_24h);
    println!("change 7d:  {}% | color=#000000", cmc_btc_ticker.percent_change_7d);

    println!("---");

    println!("Show CoinMarketCap | color=#123def href=https://coinmarketcap.com/currencies/nano/");
    println!("Show KuCoin | color=purple href=https://www.kucoin.com/#/trade.pro/XRB-BTC");
    println!("Show Binance | color=#ff9933 href=https://www.binance.com/tradeDetail.html?symbol=NANO_BTC");
    println!("Powered by Rust!");
}


fn cmc_ticker(symbol: &str) -> Result<CmcTicker, reqwest::Error> {
    let request_url = format!(
        "https://api.coinmarketcap.com/v1/ticker/{symbol}/?convert=BTC",
        symbol = symbol
    );

    let mut response = reqwest::get(&request_url)?;
    let ticker: CmcTickerResponse = response.json()?;

    Ok(ticker.content)
}

fn binance_ticker(symbol: &str) -> Result<BinanceTicker, reqwest::Error> {
    let request_url = format!(
        "https://api.binance.com/api/v1/ticker/24hr?symbol={symbol}",
        symbol = symbol
    );

    let mut response = reqwest::get(&request_url)?;
    let ticker: BinanceTicker = response.json()?;

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