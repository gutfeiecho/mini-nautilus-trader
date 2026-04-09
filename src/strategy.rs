use crate::events::{EventHandler, MarketEvent};
use std::cell::RefCell; // 用于在不可变方法中改变状态 // 引入事件相关模块

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Signal {
    Buy,
    Sell,
    Hold, // 持仓不动
}

// 策略结构体
pub struct MeanReversionStrategy {
    price_history: RefCell<Vec<f64>>,
    window_size: usize,
    threshold: f64,
}

impl MeanReversionStrategy {
    pub fn new(window_size: usize, threshold: f64) -> Self {
        Self {
            price_history: RefCell::new(Vec::with_capacity(window_size)),
            window_size,
            threshold,
        }
    }

    // 辅助方法：计算简单移动平均 (SMA)
    fn calculate_sma(&self) -> f64 {
        // 借用内部数据
        let history = self.price_history.borrow();
        if history.is_empty() {
            return 0.0;
        }
        let sum: f64 = history.iter().sum();
        sum / history.len() as f64
    }

    // 供外部查询当前信号状态
    pub fn get_signal(&self) -> Signal {
        let history = self.price_history.borrow();
        if history.is_empty() {
            return Signal::Hold;
        }

        let sma = history.iter().sum::<f64>() / history.len() as f64;
        let last_price = *history.last().unwrap();

        if last_price < sma - self.threshold {
            Signal::Buy
        } else if last_price > sma + self.threshold {
            Signal::Sell
        } else {
            Signal::Hold
        }
    }

    pub fn get_last_price(&self) -> Option<f64> {
        // 借用内部的 history
        let history = self.price_history.borrow();
        // 返回最后一个价格的副本
        history.last().copied()
    }
}

// 4. 实现 EventHandler Trait (事件驱动的核心)
impl EventHandler for MeanReversionStrategy {
    // 当事件总线推送 MarketEvent 时，这个方法会被自动调用
    fn on_event(&self, event: &MarketEvent) {
        let current_price = event.tick.price;

        // 5. 核心修改：使用 borrow_mut() 获取可变借用
        // 这相当于说：“虽然我是只读的(&self)，但我申请打开盒子修改里面的历史数据”
        let mut history = self.price_history.borrow_mut();

        // --- 策略逻辑开始 ---

        // 1. 更新历史数据 (滑动窗口)
        history.push(current_price);
        if history.len() > self.window_size {
            history.remove(0); // 移除最旧的数据
        }

        // 2. 计算指标 (SMA)
        // 注意：这里我们需要先释放上面的可变借用 history，或者直接用 self.calculate_sma()
        // 为了安全，我们显式 drop 掉 history，或者直接用 calculate_sma 方法
        drop(history);

        let sma = self.calculate_sma();
        if sma == 0.0 {
            return;
        }
    }
}
