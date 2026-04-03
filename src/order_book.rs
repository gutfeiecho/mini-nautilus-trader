use crate::types::Tick;

#[derive(Debug)]    // 宏1:自动生成Debug代码
#[allow(dead_code)] // 宏2:告诉编译器忽略“未使用字段”的警告 
pub struct OrderBook {
    pub symbol: &'static str,
    // 最新成交价
    pub last_price: f64,
    // 买一价（当前市场上有人愿意出的最高买入价）
    pub best_bid: f64,
    // 卖一价（当前市场上有人愿意接受的最低卖出价）
    pub best_ask: f64,
    // 记录更新次数
    pub updates: u64,
}

impl OrderBook {
    pub fn new(symbol: &'static str) -> Self {
        Self {
            symbol,
            last_price: 0.0,
            best_bid: 0.0,
            best_ask: 0.0,
            updates: 0,
        }
    }

    pub fn update(&mut self, tick: &Tick) {
        self.last_price = tick.price;
        self.best_bid = tick.price - 0.05;
        self.best_ask = tick.price + 0.05;
        self.updates += 1;
    }

    pub fn is_ready(&self) -> bool {
        self.last_price > 0.0
    }
}