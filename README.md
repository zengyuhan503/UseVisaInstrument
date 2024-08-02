## 电量计仪器测试应用工具

### 项目简介

本项目是基于 Tauri 和 Visa 的电量计仪器测试应用工具。它旨在为用户提供一个简单易用的界面，用于控制和监控电量计仪器。通过本工具，用户可以方便地进行电量测量、数据记录和分析。

### 主要功能

- **仪器连接**：通过 Visa 与电量计仪器建立连接。
- **实时数据监控**：实时显示电量计的电压、电流、功率等测量数据。
- **数据记录与存储**：记录测量数据，并支持历史记录回显与记录删除。
- **自动化测试**：支持设置自动化测试流程，根据预设条件自动进行测试。
- **校准功能**：提供校准工具，用于校正电量计设备的测量误差。

### 安装和使用

#### 前置条件

- 安装 [Rust](https://www.rust-lang.org/)
- 安装 [Node.js](https://nodejs.org/)
- 安装 [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites)

#### 安装步骤

1. 克隆项目仓库：

   ```bash
   git clone https://github.com/yourusername/yourproject.git
   cd yourproject
   ```

2. 安装依赖：

   ```bash
   npm install
   ```

3. 构建和运行应用：

   ```bash
   npm run tauri dev
   ```

### 使用指南

#### 连接电量计仪器

1. 打开应用并导航到连接页面。
2. 输入仪器的资源名称（如 IP 地址）并点击“连接”按钮。

#### 实时数据监控

连接成功后，应用会开始实时显示仪器的测量数据。用户可以在监控页面查看电压、电流和功率等参数的实时变化。

#### 数据记录与存储

用户可以点击查看历史记录，选择打你套数据进行回显与分析。

#### 自动化测试

1. 导航到自动化测试页面。
2. 设置测试参数和条件。
3. 点击“开始测试”按钮，应用将根据设定的条件自动进行测试并记录结果。

#### 校准功能

1. 导航到校准页面。
2. 按照提示进行校准操作。
3. 校准完成后，应用将保存校准数据，并在后续测量中应用校准结果。

### 部分代码示例

以下是一个简单的代码示例，展示了如何使用 Visa 与电量计仪器通信：

```rust
use visa_rs::{Instrument, VisaError};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), VisaError> {
    // 打开默认资源管理器
    let resource_manager = visa_rs::ResourceManager::open_default_resource_manager()?;

    // 打开电量计仪器资源
    let resource_name = "GPIB0::14::INSTR";
    let instrument = resource_manager.open_resource(resource_name)?;

    // 创建一个通道用于停止信号
    let (stop_tx, stop_rx) = mpsc::channel::<()>();

    // 启动一个线程来读取数据
    let handle = thread::spawn(move || {
        loop {
            // 检查是否接收到停止信号
            if stop_rx.try_recv().is_ok() {
                println!("Stopping data acquisition.");
                break;
            }
            
            // 读取电压数据
            instrument.write("MEASURE:VOLTAGE?").unwrap();
            let mut buffer = vec![0; 100];
            let bytes_read = instrument.read(&mut buffer).unwrap();
            let response = String::from_utf8_lossy(&buffer[..bytes_read]);
            println!("Voltage: {}", response);

            // 休眠一段时间模拟数据采集间隔
            thread::sleep(Duration::from_secs(1));
        }
    });

    // 主线程等待用户停止命令
    println!("Press Enter to stop...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    
    // 发送停止信号
    stop_tx.send(()).unwrap();
    handle.join().unwrap();

    Ok(())
}
```

### 贡献指南

欢迎任何形式的贡献！如果你有任何建议、问题或发现了 bug，请提交 issue 或 pull request。
