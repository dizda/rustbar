use redis::Connection;
use redis::Client;
use redis::Commands;
use ticker::CoinStats;
use serde_json;

pub fn connection() -> Connection {
    let client = Client::open("redis://127.0.0.1/").unwrap();
    let con = client.get_connection().unwrap();

    con
}

pub fn save(con: &Connection, name: &str, coin_stats: &CoinStats, ttl_seconds: usize) {
    // save into redis for 50s
    let _ : () = con.set_ex(name, serde_json::to_string(coin_stats).unwrap(), ttl_seconds).unwrap();
}

pub fn get(con: &Connection, name: &str) -> CoinStats {
    // get redis entry
    let temp: String = con.get(name).unwrap();
    // deserialize into CoinStats
    // convert String to &str with $* (explicit reborrowing)
    let temp: CoinStats = serde_json::from_str(&*temp).unwrap();

    temp
}
