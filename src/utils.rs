use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

/// 定义一个通用的 CSV 行结构，用于存放读取到的原始数据
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CsvRow {
    pub timestamp: u64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

/// 公共方法：读取 CSV 文件
/// 参数 path: 文件路径 (例如 "assets/BTCUSDT_1h.csv")
/// 返回: 一个迭代器，每次返回一行解析好的数据
pub fn read_csv_file<P: AsRef<Path>>(path: P) -> io::Result<impl Iterator<Item = CsvRow>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    // 1. 跳过表头 (第一行)
    if let Some(Ok(header)) = lines.next() {
        println!("📄 [Utils] CSV Header: {}", header);
    }

    // 2. 返回一个迭代器，自动处理解析逻辑
    Ok(lines.filter_map(|line_result| {
        match line_result {
            Ok(line) => {
                let parts: Vec<&str> = line.trim().split(',').collect();
                if parts.len() < 6 {
                    return None; // 格式不对跳过
                }

                // 解析各个字段
                // 对应表头: timestamp,open,high,low,close,volume
                let timestamp = parts[0].parse::<u64>().ok()?;
                let open = parts[1].parse::<f64>().ok()?;
                let high = parts[2].parse::<f64>().ok()?;
                let low = parts[3].parse::<f64>().ok()?;
                let close = parts[4].parse::<f64>().ok()?;
                let volume = parts[5].parse::<f64>().ok()?;

                Some(CsvRow {
                    timestamp,
                    open,
                    high,
                    low,
                    close,
                    volume,
                })
            }
            Err(_) => None, // 读取错误跳过
        }
    }))
}
