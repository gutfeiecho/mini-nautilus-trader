pub struct Cache {
    data: String,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            data: "Minimal Cache".to_string(),
        }
    }
    
    // 为了演示，增加一个获取数据的方法
    pub fn get_data(&self) -> &str {
        &self.data
    }
}