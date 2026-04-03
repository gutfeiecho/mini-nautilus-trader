use std::time::{SystemTime, UNIX_EPOCH};
use crate::types::Tick;

#[allow(dead_code)]
pub struct Clock {
    name: String,
    current_price: f64,
}

impl Clock {
    pub fn new(name: String) -> Self {
        Self { name, current_price: 100.0 }
    }

    // 注意：这里加上了 pub，因为 main.rs 需要调用它
    pub fn timestamp_ns(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64
    }

    // 核心方法：生成一个模拟的Tick
    pub fn generate_tick(&mut self) -> Tick {
        // 1. 模拟价格随机波动（+0.5 到 -0.5）
        let change = (rand::random::<f64>() - 0.5) * 1.0;
        self.current_price += change;

        // 2. 创建并返回Tick
        Tick::new("BTC/USDT", self.current_price, self.timestamp_ns())
    }
}