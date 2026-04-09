use std::cell::RefCell;
use std::rc::Rc;

// 1. 定义行情事件，携带 Tick 数据
#[derive(Debug, Clone)]
pub struct MarketEvent {
    pub tick: crate::types::Tick, // 依赖 types 模块
}

// 2. 定义事件处理器接口 (Trait)
// 任何想接收消息的组件（如策略）都必须实现这个 trait
pub trait EventHandler {
    // 方法接收 &self 是因为我们将使用 RefCell 来实现内部可变性
    fn on_event(&self, event: &MarketEvent);
}

// 3. 简单的消息总线
// 它持有一个处理器列表
pub struct MessageBus {
    // 使用 Rc<RefCell<>> 是为了让多个引擎（或组件）可以共享并修改订阅列表
    handlers: Vec<Rc<RefCell<dyn EventHandler>>>,
}

impl MessageBus {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }

    // 订阅方法：任何实现了 EventHandler 的对象都可以注册进来
    pub fn subscribe(&mut self, handler: Rc<RefCell<dyn EventHandler>>) {
        self.handlers.push(handler);
    }

    // 发布方法：将事件发送给所有订阅者
    pub fn publish(&self, event: &MarketEvent) {
        for handler in &self.handlers {
            // borrow_mut() 获取可变借用，调用 on_event
            // 注意：这里 on_event 定义为 &self，所以内部逻辑必须处理可变状态（如 Signal）
            handler.borrow().on_event(event);
        }
    }
}
