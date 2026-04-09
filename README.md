# mini-nautilus-trader
数据 -> 策略 -> 执行 -> 风控
低延迟、高可靠、可回测的系统

## 核心逻辑
接收数据 -> 计算指标 -> 产生信号 -> 发送订单

## 核心模块
### 数据引擎（Data-Engine）
### 执行引擎（Execution-Engine）
### 事件驱动（Event-Driven）
1. 定义事件总线（Event Bus）
2. 定义事件类型（Event）
3. Trait化策略
4. 重构内核（Kernel）

## 策略
### 均值回归（Mean Reversion）
### 动量策略（Momentum）
