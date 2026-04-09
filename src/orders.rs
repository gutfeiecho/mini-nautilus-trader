#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Order {
    pub symbol: String,
    pub side: OrderSide,
    pub quantity: i64,
    pub price: f64,
}

impl Order {
    pub fn new(symbol: &str, side: OrderSide, quantity: i64, price: f64) -> Self {
        Self {
            symbol: symbol.to_string(),
            side,
            quantity,
            price,
        }
    }
}
