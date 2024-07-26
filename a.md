---
highlight: a11y-dark
theme: github
---
> **前言**：Visa仪器电流测试工具是基于Tauri + Rust作为底层基础，使用Vue3作为Ui开发的工具，包含了电流单通道、多通道模式下对不同参数的电流电压进行测试


#### 一、安装Visa的依赖环境

 在电子测试中，Visa是一个非常关键的标准，他提供了通用的接口，用于控制和通信各种测试设备，如示波器、电源供应器、频谱分析器等。
主要用于设备的同学、设备控制、数据采集、错误处理。让你用标准的方法来简化和优化设备的控制和数据的采集。

1、安装Visa的依赖环境
    使用官方环境进行安装：Ni.com/zh-cn/support/downloads/drivers/download.ni-visa.html#521671
    1）下载ni-visa_24.0_online.exe安装包，安装即可。







#### 二、使用Tauri

1、安装Tauri ：https://tauri.app/v1/api/js/tauri/#convertfilesrc
```shell
    npm create tauri-app@latest
    ...
    ...
    cd your-project-name
    npm install
    npm run tauri dev
```

2、使用Tauri,配置cargo.toml，使用visa-rs 与仪器进行连接通信的操作。
它的地址是：https://crates.io/crates/visa-rs


