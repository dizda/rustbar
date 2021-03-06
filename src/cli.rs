use ticker;
use util::thousands;
use std::error::Error;
use osascript::JavaScript;
use math::Math;

#[derive(Serialize)]
struct TouchBarParams {
    widget_uuid: String,
    label: String
}

pub fn print_to_touch_bar(amount: String) {
    let widget_btc = String::from("1A9010BF-D26E-4016-BD99-5D78CA8496FF");
    let widget_nano = String::from("92B67153-3DF5-4C7A-9710-4E2AD52C0C88");

    // acquire the lock on the global static in order to be thread-safe
    let coin_stats = ticker::COIN_STATS.lock().unwrap();

    update_touch_bar(widget_btc, &coin_stats.bitcoin.price_usd, amount.clone());
    update_touch_bar(widget_nano, &coin_stats.last_price_usd, amount);
}

fn update_touch_bar(widget_uuid: String, ticker_price: &String, amount: String) {
    let script = JavaScript::new("
        var BetterTouchTool = Application('BetterTouchTool');

        BetterTouchTool.update_touch_bar_widget($params.widget_uuid,
        {
            text: $params.label
        });
    ");

    let mut amount = amount;
    // strip "," from thousands if any
    amount.retain(|c| c != ',');


    if !check_if_number(amount.clone()) { // clone the value because it goes to a different scope which won't return the value
        // if it's not a number we exit
        amount = String::from("Error!");
    } else {
        amount = String::from(amount).multiply(&ticker_price, 2);
        amount = "$".to_owned() + &thousands(&amount, 2);
    }

    script.execute_with_params(TouchBarParams {
        widget_uuid,
        label: amount
    }).unwrap_or_else(|err| {
        eprintln!("Problem sending AppleScript: {}", err);
    });
}


/**
 * Used by BitBar
 */
pub fn print_to_stdout() -> Result<(), Box<dyn Error>> {
    let img_up = "iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAABmJLR0QAyQACAALwzISXAAAACXBIWXMAAAsTAAALEwEAmpwYAAAAB3RJTUUH4AQHACkSBTjB+AAAALNJREFUOMvVk70NAjEMhb87WYiGBZAQU7ABNSVSWpZgEEagsJDoKBELUCEKFuBuCKTw0xyQC0lICe5i+/k9/wT+3opUUJQhcAUqa8I5ZQT4tANwioGTCkQZA9vmOQE2oUJFhL0DXBz33RpKUfCLfLTQJMx9IlEWuQr6QB3prGtNS1lwiMvEYo7ekNsKRBkB+y+rH1hDFVOwy7ids+gbVzrsM6CXeYDTF85xroB1ZoHb73ymB5RhJkpZTihGAAAAAElFTkSuQmCC";
    let img_down = "iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAABmJLR0QABACnAADQ9FZaAAAACXBIWXMAAAsTAAALEwEAmpwYAAAAB3RJTUUH4AQHACQ1FZwK3gAAAMRJREFUOMvNkjEKAjEQRZ+jKNjYKh5AbzCdjVcQj+BFPIKlp7EMeAJrUbASQVCEr80uG9cNbqe/Cgn/5WUI/DqNfBHM+kCzbs+lPUAr2pwBq5qABbB+M8gszkDvS/kOdAG5VBgEM4ApsP0CGLukjxlEoA0wSZR3Lo0qhxhZDIBDAmDA0wsBLD51CZeOwLKivHbprZx6AkAHuEXbD5fawYwywMqAzOKeDTTPvKqcTGZBMLsGs0utn5gADYEHcKp9e9ni//MCDtNCE3qjsIwAAAAASUVORK5CYII=";
    let img: &str;

    // generate stats, if any errors, they will be delegated to the caller (main.rs)
    ticker::get_stats()?;

    // then get the result from the global variable (because it has been generated by a different thread
    let stats = ticker::COIN_STATS.lock().unwrap();

    // cast to float
    let percent_change_1h: f32 = stats.change1h.parse().unwrap();

    if percent_change_1h > 0. {
        img = img_up;
    } else {
        img = img_down;
    }

    println!("{} (${}) | image={}", stats.last_price_btc, stats.last_price_usd, img);

    println!("---");

    println!("NANO | image=iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAMAAAAoLQ9TAAAABGdBTUEAALGPC/xhBQAAAAFzUkdCAK7OHOkAAABRUExURUxpcVSJykmQ4kaS3lKLzWyUxESF2V+W0FKV7lGJyU6O4EmR3k6P3lSU6EyP3kmT30qP4keU3kCT5kWS4UiR4VKL4EOT40OQ6ESS5UWQ5kiQ4v2G/RwAAAAKdFJOUwDr///+EP4F/tfCPIxYAAAAQ0lEQVQY02NgoBngkoAy+ATBFIukFITPzs0LJDlEGBkYmJiAXGFxsDAHMztEgJNZFMQVgGpnFWMD0/w8UAEhLto5GgDaMgGRTwkAAgAAAABJRU5ErkJggg==");
    println!("buy: {} | color=green", stats.buy_btc);
    println!("sell: {} | color=red", stats.sell_btc);
    println!("vol: {} BTC (${})", stats.volume_btc, stats.volume_usd);
    println!("change-24h: {}%", stats.change24h);
    println!("high: {} (${})", stats.high_btc, stats.high_usd);
    println!("low: {} (${})", stats.low_btc, stats.low_usd);
    println!("price: ${}", stats.price);
    println!("rank: #{}", stats.rank);

    println!("---");


    println!("NANO daily-trading | image=iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAMAAAAoLQ9TAAAABGdBTUEAALGPC/xhBQAAAAFzUkdCAK7OHOkAAABRUExURUxpcVSJykmQ4kaS3lKLzWyUxESF2V+W0FKV7lGJyU6O4EmR3k6P3lSU6EyP3kmT30qP4keU3kCT5kWS4UiR4VKL4EOT40OQ6ESS5UWQ5kiQ4v2G/RwAAAAKdFJOUwDr///+EP4F/tfCPIxYAAAAQ0lEQVQY02NgoBngkoAy+ATBFIukFITPzs0LJDlEGBkYmJiAXGFxsDAHMztEgJNZFMQVgGpnFWMD0/w8UAEhLto5GgDaMgGRTwkAAgAAAABJRU5ErkJggg==");

    println!("swing: {} (${})", stats.daily_trading.spread_btc, stats.daily_trading.spread_usd);
    println!("possible gain: {} BTC (${})", stats.daily_trading.possible_gain_btc, stats.daily_trading.possible_gain_usd);

    println!("---");

    println!("Bitcoin | image=iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAC3klEQVR4nGWTTWicVRSGn3Pv/b75sWkSaTQmNSWgiHZjS6DGFCd2pRTcaDcRFy5MaNq6UXCnuNdFF7ZaEUFUsCgIWkTB2lmISN0IdiH+1AaZSir9SSeZmfvde46LQVRcHDiL8/DCeXkEwAwRwQDih48ueO+XVa2l2aYAxEuncHKuyvnN8onPvv43I38vF99u1XeONF4RZMUXEiwqKSuYEooSKYSccjKzU+Fm/zl5ut03Q8ROH/JMrBfVleJMsa08MLg+MEwVcOKCEOpY76ohTvE1VxsrperGs8VEdZArt1UC0H938URtvHZ4cLUfRSgJdYg3kcm9uJmHscvnIfXJnW/B12NtvCzjjXiitvTVEYnv798H7htTU8N5qi7uzocIe1dBPDa4jtRGSd8dR3/7EpoTJpZVnDjQeafZjhXBRHMCrSA00LU2utZGxmYBgcat+PuXkTvmIHZFVRkydsyhqaWxAlOHZTDFqk2oj0PqET8+RPzgEdyO3RQLL4JGsOw0VqDVYkDzZKUgMCxSDClHcJNz4MshlPqQ+uj696AJMZUqKWY26UyTDJMzAGytI+UIsn2a/Ovn2GADv3sJLJMvvAdmw1sdjhPLfxSimKlRbeKm5/F7VkAc+Yd3qD49SvzkKQgN/H1LUG1hqlaI4tDLAdNzLvAkfVPywMv0PG7nfkAID7yAv+sx3O17QBx27WcwBc3qSvPW17bEt+7dh9mwRgnOuh3x9zxOmHsWi11kbBa79gv25wXS+ePDHziXxeEq5UEB2Hr97tcao361dyNHUq/0My0ITfxMi/zjR+SLXyC17RCaGBIbY6HsbeSTzZWfVsVO49ncVcQYzpTb5EBvw4zUV0ydNHcIqYcZBiimrjHqJHbtbFmmg9xyqfpHppd21aem7VVnLIfgQ06QqgrEEbzgCyElS8Abv3d4fvblS0OZ/qfzyZkF7/WZlFnUzBQY3ktHHG1Td6o8vPYfnf8CyHGVDNs26GQAAAAASUVORK5CYII=");

    println!("price:  ${}", thousands(&stats.bitcoin.price_usd, 2));
    println!("change 1h:  {}%", stats.bitcoin.percent_change_1h);
    println!("change 24h:  {}%", stats.bitcoin.percent_change_24h);
    println!("change 7d:  {}%", stats.bitcoin.percent_change_7d);

    println!("---");

    println!("Show CoinMarketCap | color=#123def href=https://coinmarketcap.com/currencies/nano/");
    println!("Show KuCoin | color=purple href=https://www.kucoin.com/#/trade.pro/XRB-BTC");
    println!("Show Binance | color=#ff9933 href=https://www.binance.com/tradeDetail.html?symbol=NANO_BTC");
    println!("Powered by Rust!");

    Ok(())
}

fn check_if_number(number: String) -> bool {
    number.trim().parse::<f64>().is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_if_number_asserts() {
        assert!(check_if_number(String::from("33")));
        assert!(check_if_number(String::from("121200.21")));
        assert!(!check_if_number(String::from("qdqd")));
        assert!(check_if_number(String::from("0.000021")));
        assert!(!check_if_number(String::from("333,333.22")));
    }
}