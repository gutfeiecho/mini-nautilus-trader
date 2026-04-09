#[derive(Debug, Clone, Copy)]
pub struct Tick {
    /*
     * &str: 字符串切片
     * 'static: 永恒的声明周期
     */
    pub symbol: &'static str, // 交易对名称，如“BTC/USDT”
    pub price: f64,           // 价格
    pub timestamp: u64,       // 收到数据的时间戳
}

impl Tick {
    pub fn new(symbol: &'static str, price: f64, timestamp: u64) -> Self {
        Self {
            symbol,
            price,
            timestamp,
        }
    }
}
