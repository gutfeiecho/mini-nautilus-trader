use crate::orders::Order;
use crate::orders::OrderSide;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Portfolio {
    pub cash: f64,     // 现金
    pub position: i64, // 持仓数量 (正数为多，负数为空)
    pub symbol: String,
}

impl Portfolio {
    pub fn new(initial_cash: f64, symbol: &str) -> Self {
        Self {
            cash: initial_cash,
            position: 0,
            symbol: symbol.to_string(),
        }
    }

    // 核心方法：处理订单成交
    pub fn on_order_filled(&mut self, order: &Order) {
        let cost = order.price * order.quantity as f64;

        match order.side {
            OrderSide::Buy => {
                if self.cash >= cost {
                    self.cash -= cost;
                    self.position += order.quantity;
                    println!(
                        "💰 [Portfolio] Bought {} @ {:.2}. Cash: {:.2}, Pos: {}",
                        order.quantity, order.price, self.cash, self.position
                    );
                } else {
                    println!("❌ [Portfolio] Insufficient cash to buy!");
                }
            }
            OrderSide::Sell => {
                // 简单的现货逻辑：不能卖空（除非你允许 position < 0）
                if self.position >= order.quantity {
                    self.cash += cost;
                    self.position -= order.quantity;
                    println!(
                        "💰 [Portfolio] Sold {} @ {:.2}. Cash: {:.2}, Pos: {}",
                        order.quantity, order.price, self.cash, self.position
                    );
                } else {
                    println!("❌ [Portfolio] Insufficient position to sell!");
                }
            }
        }
    }
}
