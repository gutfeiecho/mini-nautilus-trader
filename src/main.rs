use std::{
    cell::RefCell,
    rc::Rc,
};

// 声明模块
mod clock;
mod cache;
mod types;
mod order_book;
mod strategy;

// 引入模块中的具体内容
use clock::Clock;
use cache::Cache;
use order_book::OrderBook;
use strategy::{Strategy, MeanReversionStrategy, Signal};

// 1. 定义最基础的数据结构
// 模拟数据引擎
struct DataEngine {
    is_running: bool,
    clock: Rc<RefCell<Clock>>,
    order_book: Rc<RefCell<OrderBook>>
}

impl DataEngine {
    fn new(clock: Rc<RefCell<Clock>>, order_book: Rc<RefCell<OrderBook>>) -> Self {
        Self {
            is_running: false,
            clock,
            order_book,
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
        println!("📈 [DataEngine] Received Tick: {} @ {} (at {})", tick.symbol, tick.price, tick.timestamp);
        
        // 2. 更新订单簿
        self.order_book.borrow_mut().update(&tick);

        // 3. 打印详细日志
        let ob = self.order_book.borrow();
        if ob.is_ready() {
            println!("📈 [DataEngine] Tick: {} | Last: {:.2} | Bid: {:.2} | Ask: {:.2}", tick.symbol, ob.last_price, ob.best_bid, ob.best_ask);
        }
    }
}

// 模拟执行引擎
struct ExecutionEngine {
    is_running: bool,
    /*
    * Rc: 引用计数，负责多人共享
    * RefCell: 内部可变形的安全箱。它在运行时检查借用规则。
    */
    cache: Rc<RefCell<Cache>>,
}

impl ExecutionEngine {
    fn new(cache: Rc<RefCell<Cache>>) -> Self {
        Self {
            is_running: false,
            cache,
        }
    }
    // 参数&mut self: 给方法“修改权”
    fn start(&mut self) {
        self.is_running = true;
        // self.cache.borrow(): 打开“安全箱”
        // borrow()是RefCell提供的一个方法。它的作用是“我要打开这个箱子，借用里面的Cache对象。”
        // 返回一个智能指针（类型叫Ref<Cache>）
        let cache_ref = self.cache.borrow(); // 1. 拿到钥匙，并保存起来
        let data = cache_ref.get_data();     // 2. 用钥匙开门，拿到数据
        println!("[ExecutionEngine] Started, reading cache: {}", data);
    }

    fn stop(&mut self) {
        self.is_running = false;
        println!("[ExecutionEngine] Stopped");
    }
}

// 2. 实现最小化的 Nautilus Kernel
#[allow(dead_code)]
struct MiniKernel {
    // 核心组件
    clock: Rc<RefCell<Clock>>,
    cache: Rc<RefCell<Cache>>,
    
    // 引擎
    data_engine: DataEngine,
    exec_engine: ExecutionEngine,

    // 订单簿
    order_book: Rc<RefCell<OrderBook>>,

    // 策略模块
    strategy: Box<dyn Strategy>, // 使用Box来持有trait对象

    // 状态
    ts_started: Option<u64>,
}

impl MiniKernel {
    // 构造函数：组装所有部件
    pub fn new() -> Self {
        println!("Building MiniKernel...");

        // 1. 创建基础服务
        let clock = Rc::new(RefCell::new(Clock::new("TestClock".to_string())));
        let cache = Rc::new(RefCell::new(Cache::new()));

        // 2. 创建订单簿
        let order_book = Rc::new(RefCell::new(OrderBook::new("BTC/USDT")));

        // 3. 将基础服务注入引擎
        let data_engine = DataEngine::new(clock.clone(), order_book.clone());
        let exec_engine = ExecutionEngine::new(cache.clone());

        // 4. 初始化策略
        let strategy = Box::new(MeanReversionStrategy::new(5, 0.2));

        Self {
            clock,
            cache,
            data_engine,
            exec_engine,
            order_book,
            strategy,
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

            // 策略思考
            // 借用订单簿，传给策略
            let ob_ref = self.order_book.borrow();

            println!("   🧠 [Strategy: {}] Processing...", self.strategy.name());
            let signal = self.strategy.on_market_data(&ob_ref); 

            if signal != Signal::Hold {
                println!("   🚀 [Kernel] Executing signal: {:?}", signal);
            }
            // 释放借用
            drop(ob_ref);

            std::thread::sleep(std::time::Duration::from_millis(100));
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

// 3. 主函数
fn main() {
    // 创建内核实例
    let mut kernel = MiniKernel::new();
    
    // 启动系统
    kernel.start();
    
}
