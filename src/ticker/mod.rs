pub mod binance;
pub mod cmc;

use math::Math;
use ticker::binance::BinanceTicker;
use ticker::cmc::*;
use util::thousands;
use std::error::Error;
use std::sync::Mutex;

lazy_static! {
    // Default initialize the struct with empty values.
    pub static ref COIN_STATS: Mutex<CoinStats> = Mutex::new(CoinStats::default());
}

// NANO compiled stats
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct CoinStats {
    pub last_price_btc: String,
    pub last_price_usd: String,
    pub buy_btc: String,
    pub sell_btc: String,
    pub volume_btc: String,
    pub volume_usd: String,
    pub change1h: String,
    pub change24h: String,
    pub high_btc: String,
    pub high_usd: String,
    pub low_btc: String,
    pub low_usd: String,
    pub price: String,
    pub rank: String,
    pub daily_trading: DailyTrading,
    pub bitcoin: CmcTicker,
}

// daily trading calculation
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct DailyTrading {
    pub spread_btc: String,
    pub spread_usd: String,
    pub possible_gain_btc: String,
    pub possible_gain_usd: String
}

// Get tickers then save them in a global static
pub fn get_stats() -> Result<(), Box<dyn Error>> {
    // overwrite the global static
    *COIN_STATS.lock().unwrap() = compute()?;

    Ok(())
}

// Build the struct
fn compute() -> Result<CoinStats, Box<dyn Error>> {
    // unwrap return the "Ok" part
    let binance_nano_ticker = BinanceTicker::ticker("NANOBTC")?;
    let cmc_nano_ticker = CmcTicker::ticker("nano")?;
    let cmc_btc_ticker = CmcTicker::ticker("bitcoin")?;

    // Calculate
    let spread_24h_btc = binance_nano_ticker.high_price.sub(&binance_nano_ticker.low_price, 8);
    let spread_24h_usd = thousands(&spread_24h_btc.multiply(&cmc_btc_ticker.price_usd, 2), 2);

    // gain if we swingtrade with 100k NANO
    let possible_gain_btc = spread_24h_btc.multiply(&100000_f64.to_string(), 2);
    let possible_gain_usd = thousands(&possible_gain_btc.multiply(&cmc_btc_ticker.price_usd, 8), 2);

    let daily_trading = DailyTrading {
        spread_btc: spread_24h_btc,
        spread_usd: spread_24h_usd,
        possible_gain_btc,
        possible_gain_usd
    };

    let ticker = CoinStats {
        last_price_btc: binance_nano_ticker.last_price,
        last_price_usd: thousands(&cmc_nano_ticker.price_usd, 2),
        buy_btc: binance_nano_ticker.bid_price,
        sell_btc: binance_nano_ticker.ask_price,
        volume_btc: thousands(&cmc_nano_ticker.last_24h_volume_btc, 0),
        volume_usd: thousands(&cmc_nano_ticker.last_24h_volume_usd, 0),
        change1h: cmc_nano_ticker.percent_change_1h,
        change24h: cmc_nano_ticker.percent_change_24h,
        high_btc: binance_nano_ticker.high_price.to_string(),
        high_usd: thousands(&cmc_btc_ticker.price_usd.multiply(&binance_nano_ticker.high_price, 2), 2),
        low_btc: binance_nano_ticker.low_price.to_string(),
        low_usd: thousands(&cmc_btc_ticker.price_usd.multiply(&binance_nano_ticker.low_price, 2), 2),
        price: thousands(&cmc_nano_ticker.price_usd, 2),
        rank: cmc_nano_ticker.rank,
        bitcoin: cmc_btc_ticker,
        daily_trading
    };

    Ok(ticker)
}