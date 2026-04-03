use crate::order_book::OrderBook;

#[derive(Debug, PartialEq)]
pub enum Signal {
    Buy,
    Sell,
    Hold, // 持仓不动
}

pub trait Strategy {
    fn name(&self) -> &str;
    fn on_market_data(&mut self, ob: &OrderBook) -> Signal;
}

pub struct MeanReversionStrategy {
    price_history: Vec<f64>,
    window_size: usize,
    threshold: f64,
}

impl MeanReversionStrategy {
    pub fn new(window_size: usize, threshold: f64) -> Self {
        Self {
            price_history: Vec::with_capacity(window_size),
            window_size,
            threshold,
        }
    }

    // 辅助方法：计算简单移动平均
    fn calculate_sma(&self) -> f64 {
        if self.price_history.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.price_history.iter().sum();
        sum / self.price_history.len() as f64
    }
}

impl Strategy for MeanReversionStrategy {
    fn name(&self) -> &str {
        "MeanReversion_v1"
    }

    fn on_market_data(&mut self, ob: &OrderBook) -> Signal {
        // 1. 数据清洗：如果价格无效，直接忽略
        if !ob.is_ready() {
            return Signal::Hold;
        }

        let current_price = ob.last_price;

        // 2. 更新历史数据
        self.price_history.push(current_price);
        if self.price_history.len() > self.window_size {
            self.price_history.remove(0); // 移除最旧的数据，保持窗口大小
        }

        // 3. 计算指标
        let sma = self.calculate_sma();

        if sma == 0.0 {
            return Signal::Hold;
        }

        // 4. 产生信号
        // 价格低于均线 - 阈值 -> 买入（抄底）
        if current_price < sma - self.threshold {
            println!("   🧠 [Strategy] Price {:.2} is LOW (SMA: {:.2}). Signal: BUY", current_price, sma);
            return Signal::Buy;
        }
        // 价格高于均线 + 阈值 -> 卖出（逃顶）
        else if current_price > sma + self.threshold {
            println!("   🧠 [Strategy] Price {:.2} is HIGH (SMA: {:.2}). Signal: SELL", current_price, sma);
            return Signal::Sell;
        }

        Signal::Hold
    }
}