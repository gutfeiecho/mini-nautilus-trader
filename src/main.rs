// 声明模块
mod clock;
mod events;
mod order_book;
mod orders;
mod portfolio;
mod strategy;
mod types;
mod utils;

// 引入模块中的具体内容
use clock::Clock;
use events::{MarketEvent, MessageBus};
use order_book::OrderBook;
use orders::{Order, OrderSide};
use portfolio::Portfolio;
use std::{cell::RefCell, rc::Rc};
use strategy::{MeanReversionStrategy, Signal};
use utils::{CsvRow, read_csv_file};

// 定义数据引擎
struct DataEngine {
    is_running: bool,
    clock: Rc<RefCell<Clock>>,
    message_bus: Rc<RefCell<MessageBus>>,
}

impl DataEngine {
    fn new(clock: Rc<RefCell<Clock>>, message_bus: Rc<RefCell<MessageBus>>) -> Self {
        Self {
            is_running: false,
            clock,
            message_bus,
        }
    }

    fn start(&mut self) {
        self.is_running = true;
        println!("[DataEngine] Started");
    }

    fn stop(&mut self) {
        self.is_running = false;
        println!("[DataEngine] Stopped");
    }

    pub fn process_tick(&self) {
        // 1. 借用clock，生成一个tick
        let tick = self.clock.borrow_mut().generate_tick();
        println!(
            "📈 [DataEngine] Received Tick: {} @ {} (at {})",
            tick.symbol, tick.price, tick.timestamp
        );

        // 2. 构造事件
        let event = MarketEvent { tick };

        // 3. 发布事件到总线
        // 这里总线会通知所有订阅者（策略）
        self.message_bus.borrow().publish(&event);
    }
}

// 定义执行引擎
struct ExecutionEngine {
    is_running: bool,
    /*
     * Rc: 引用计数，负责多人共享
     * RefCell: 内部可变形的安全箱。它在运行时检查借用规则。
     */
    portfolio: Rc<RefCell<Portfolio>>,
}

impl ExecutionEngine {
    fn new(portfolio: Rc<RefCell<Portfolio>>) -> Self {
        Self {
            is_running: false,
            portfolio,
        }
    }
    // 参数&mut self: 给方法“修改权”
    fn start(&mut self) {
        self.is_running = true;
        // self.cache.borrow(): 打开“安全箱”
        // borrow()是RefCell提供的一个方法。它的作用是“我要打开这个箱子，借用里面的Cache对象。”
        // 返回一个智能指针（类型叫Ref<Cache>）
        // let cache_ref = self.cache.borrow(); // 1. 拿到钥匙，并保存起来
        // let data = cache_ref.get_data();     // 2. 用钥匙开门，拿到数据
        // println!("[ExecutionEngine] Started, reading cache: {}", data);
        println!("[ExecutionEngine] Started");
    }

    pub fn submit_order(&self, order: &Order) {
        println!("   📝 [ExecEngine] Processing Order: {:?}", order);
        // 模拟撮合：假设订单立即以当前价格成交（市价单）
        // 在真实系统中，这里会去匹配 OrderBook
        self.portfolio.borrow_mut().on_order_filled(order);
    }

    fn stop(&mut self) {
        self.is_running = false;
        println!("[ExecutionEngine] Stopped");
    }
}

// 实现最小化的 Nautilus Kernel
#[allow(dead_code)]
struct MiniKernel {
    // 核心组件
    clock: Rc<RefCell<Clock>>,

    // 数据引擎
    data_engine: DataEngine,

    // 执行引擎
    exec_engine: ExecutionEngine,

    // 订单簿
    order_book: Rc<RefCell<OrderBook>>,

    // 事件总线
    message_bus: Rc<RefCell<MessageBus>>,

    // 策略
    strategy: Rc<RefCell<MeanReversionStrategy>>,

    // 状态
    ts_started: Option<u64>,

    // 账户
    portfolio: Rc<RefCell<Portfolio>>,
}

impl MiniKernel {
    // 构造函数：组装所有部件
    pub fn new() -> Self {
        // 1. 创建基础服务
        let clock = Rc::new(RefCell::new(Clock::new("TestClock".to_string())));
        let message_bus = Rc::new(RefCell::new(MessageBus::new()));

        // 2. 创建订单簿
        let order_book = Rc::new(RefCell::new(OrderBook::new("BTC/USDT")));

        // 3. 初始化策略
        let strategy = Rc::new(RefCell::new(MeanReversionStrategy::new(5, 0.2)));

        message_bus.borrow_mut().subscribe(strategy.clone());

        // 4. 将基础服务注入引擎
        let portfolio = Rc::new(RefCell::new(Portfolio::new(10000.0, "BTC/USDT")));
        let data_engine = DataEngine::new(clock.clone(), message_bus.clone());
        let exec_engine = ExecutionEngine::new(portfolio.clone());

        Self {
            clock,
            data_engine,
            exec_engine,
            portfolio,
            order_book,
            strategy,
            message_bus,
            ts_started: None,
        }
    }

    // 启动内核
    pub fn start(&mut self) {
        println!("🚀 MiniKernel Starting...");

        // 1. 启动引擎（真实的 Nautilus 会在这里处理错误）
        self.data_engine.start();
        self.exec_engine.start();

        // 2. 记录启动时间
        self.ts_started = Some(self.clock.borrow().timestamp_ns());
        println!("🎉 MiniKernel Started! Entering Event Loop...");

        // 3. 模拟事件循环
        for _i in 1..=20 {
            self.data_engine.process_tick();

            let signal = self.strategy.borrow().get_signal();

            let current_price = self.strategy.borrow().get_last_price().unwrap_or(0.0);

            let side = if signal == Signal::Buy {
                OrderSide::Buy
            } else {
                OrderSide::Sell
            };
            let order = Order::new("BTC/USDT", side, 1, current_price);

            self.exec_engine.submit_order(&order);
        }

        println!("🛑 Loop finished, stopping kernel...");
        self.stop();
    }

    // 停止内核
    pub fn stop(&mut self) {
        println!("🛑 MiniKernel Stopping...");

        // 1. 停止引擎
        self.exec_engine.stop();
        self.data_engine.stop();

        println!("👋 MiniKernel Stopped.");
    }
}

// 主函数
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 测试读取csv数据
    let rows: Vec<CsvRow> = read_csv_file("assets/BTCUSDT_1h.csv")?.collect();
    println!("📚 [Main] Loaded {} rows from CSV", rows.len());

    // 创建内核实例
    let mut kernel = MiniKernel::new();

    // 启动系统
    kernel.start();

    Ok(())
}
