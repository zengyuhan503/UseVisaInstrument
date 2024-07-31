use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader};

use std::{
    ffi::CString,
    io::Write,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::sync::mpsc::Receiver;
use tokio::time::sleep;
use visa_rs::VisaString;
use visa_rs::{flags::AccessMode, AsResourceManager, DefaultRM, Instrument, TIMEOUT_IMMEDIATE};
lazy_static! {
    static ref RESOURCE_DEFAULTRM: Arc<Mutex<DefaultRM>> = Arc::new(Mutex::new(
        DefaultRM::new()
            .map_err(|err| {
                log::error!("{:?}", err);
                return format!("{:?}", err);
            })
            .unwrap()
    ));
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ItemMax {
    max: f64,
    min: f64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct ParamsItem {
    volt: f64,
    curr: f64,
    case: bool,
    switch: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct SetInstrParams {
    CH1: ParamsItem,
    CH2: ParamsItem,
    CH3: ParamsItem,
}

// 定义一个迭代器结构体
struct SetInstrParamsIntoIterator {
    set_instr_params: SetInstrParams,
    index: usize,
}
impl IntoIterator for SetInstrParams {
    type Item = ParamsItem;
    type IntoIter = SetInstrParamsIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        SetInstrParamsIntoIterator {
            set_instr_params: self,
            index: 0,
        }
    }
}

impl Iterator for SetInstrParamsIntoIterator {
    type Item = ParamsItem;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.index {
            0 => Some(self.set_instr_params.CH1.clone()),
            1 => Some(self.set_instr_params.CH2.clone()),
            2 => Some(self.set_instr_params.CH3.clone()),
            _ => None,
        };
        self.index += 1;
        result
    }
}

#[derive(Debug)]
pub struct VisaInstrument {
    instr: Instrument,
}

/**
 * VisaInstrument
 * 创建一个VisaInstrument实例
 * @param instr_ip: 设备ip
 */
impl VisaInstrument {
    /// 创建一个新的Visa 仪器实例。
    ///
    /// 这个函数尝试根据提供的仪器IP地址初始化一个DEFAULTR资源管理器的实例。
    /// 它首先尝试获取资源管理器的锁，然后根据IP地址构造一个资源表达式，
    /// 接着查找和打开相应的资源，最后返回一个新的DEFAULTR实例。
    ///
    /// # 参数
    /// `instr_ip` - 仪器的IP地址，用于构造资源表达式。
    ///
    /// # 返回值
    /// 返回一个`Result`类型，其中包含初始化成功的DEFAULTR实例或错误信息。
    pub fn new(instr_ip: String) -> Result<Self, String> {
        // 日志记录初始化开始
        log::info!("初始化DEFAULTR 实例");
        // 尝试获取资源管理器的锁，如果失败，则格式化错误并返回
        let rm = RESOURCE_DEFAULTRM.lock().map_err(|e| format!("{:?}", e))?; 
        // 日志记录创建CString实例开始
        log::info!("创建CString 实例");
        // 构造资源表达式
        let ip_s = format!("TCPIP0::{}::inst0::INSTR", instr_ip);
        // 尝试将构造的字符串转换为CString，如果失败，则格式化错误并返回
        let expr = CString::new(ip_s)
            .map_err(|err| format!("没有找到相关资源：{:?}", err))?
            .into();

        // 在资源管理器中查找资源，如果失败，则格式化错误并返回
        let rsc = rm
            .find_res(&expr)
            .map_err(|op| format!("未找到相关的资源{:?}", op))?;

        // 使用找到的资源打开仪器，如果失败，则格式化错误并返回
        let instr = rm
            .open(&rsc, AccessMode::NO_LOCK, TIMEOUT_IMMEDIATE)
            .map_err(|op| format!("{:?}", op))?;

        // 返回初始化成功的DEFAULTR实例
        Ok(Self { instr })
    }
    /// 查找所有资源的函数
    /// 
    /// 返回结果为包含VisaString类型资源列表的Result对象，如果发生错误，则错误信息为String类型。
    pub fn find_all_resources() -> Result<Vec<VisaString>, String> {
        // 尝试获取RESOURCE_DEFAULTRM的锁，如果失败，则转换错误信息并返回
        let rm = RESOURCE_DEFAULTRM.lock().map_err(|e| format!("{:?}", e))?; 
        // 创建CString对象，用于后续的资源查找，如果创建失败，则转换错误信息并返回
        let expr = CString::new("?*INSTR").map_err(|err| format!("没有找到相关资源：{:?}", err))?; 
        // 初始化一个空的VisaString向量，用于存放查找结果
        let mut visa_list: Vec<VisaString> = Vec::new();
        // 使用rm对象和表达式查找资源列表
        let list = rm.find_res_list(&expr.into());
        // 根据查找结果进行处理
        match list {
            // 如果查找成功，遍历结果列表
            Ok(list) => {
                for item in list {
                    // 对于每个查找结果项，根据结果进行处理
                    match item {
                        // 如果结果项成功，将其添加到visa_list中
                        Ok(vs) => {
                            visa_list.push(vs);
                        }
                        // 如果结果项失败，打印错误信息
                        Err(err) => {
                            println!("error:{:?}", err)
                        }
                    }
                }
            }
            // 如果查找失败，返回错误信息
            Err(err) => {
                return Err(format!("未找到相关的资源{:?}", err));
            }
        }
        // 打印查找结果
        println!("visa_list:{:?}",visa_list);
        // 返回包含查找结果的Ok对象
        Ok(visa_list)
    }
    /// 异步关闭所有设备。
    ///
    /// 此方法通过发送一条指令来关闭设备输出，然后清除指令缓冲区。
    /// 它返回一个结果，指示关闭操作是否成功。
    pub async fn close_all(&mut self) -> Result<(), String> {
        // 构造关闭输出的指令字符串
        let cmds = format!("OUTP OFF,(@1:3)\n");
        // 将指令字符串转换为字节序列，以备写入
        let cmd = cmds.as_bytes();
        // 获取指令写入器的引用
        let mut write_instr = &self.instr;
        // 尝试写入指令，并处理可能的错误
        let res = write_instr.write_all(cmd).map_err(|err| format!("{}", err));
        // 清除指令缓冲区，确保后续操作不会受到本次操作的影响
        let _ = self.instr.clear();
        // 返回操作结果
        res
    }
    pub fn set_config(&mut self, item: String) -> Result<(), String> {
        let mut write_instr = &self.instr;

        // 解析item带来的配置信息
        let configs: SetInstrParams = serde_json::from_str(&item)
            .map_err(|err| {
                format!("{}", err)
            })
            .unwrap();
        for (index, config) in configs.into_iter().enumerate() {
            if config.case {
                let chn = (index + 1).to_string();
                let switch = config.switch;
                let is_oepn = if switch { "ON" } else { "OFF" };
                let cmds = format!("OUTP {},(@{})\n", is_oepn, chn);
                let cmd = cmds.as_bytes();
                let _write = write_instr.write_all(cmd);

                let set_chn = format!("InST:NSEL {}\n", chn);
                let _is_set_chn = write_instr.write_all(set_chn.as_bytes()).map_err(|err| {
                    return format!("error:{:?}", err);
                })?;
                let volt = config.volt;
                let set_volt = format!("VOLT {}\n", volt);
                let _is_set_volt = write_instr.write_all(set_volt.as_bytes()).map_err(|err| {
                    return format!("error:{:?}", err);
                })?;

                let curr = config.curr;
                let set_curr = format!("CURR {}\n", curr);
                let _is_set_curr = write_instr.write_all(set_curr.as_bytes()).map_err(|err| {
                    return format!("error:{:?}", err);
                })?;

                log::info!("设置通道{},volt:{},curr:{}", index + 1, volt, curr);
            }
        }

        Ok(())
    }
    pub async fn _start_runing(&mut self) {}
    pub async fn _stop_runing(&mut self) {}

    pub async fn read_current_data(
        &self,
        send_win: tauri::Window,
        mut stop_rx: Receiver<()>,
        item: String,
        split_time: String,
    ) {
        let chns: Vec<String> = serde_json::from_str(&item).unwrap();
        let rate = split_time.parse().unwrap();
        let interval = Duration::from_millis(rate);
        let _ = send_win.emit("start_read_case", "start");
        loop {
            tokio::select! {
                _ = stop_rx.recv() => {
                    println!("Received stop signal, stopping loop.");
                    break;
                },
                _ = sleep(interval) => {

                    let chns =chns.clone();
                    let mut res = String::new();
                    for chn in chns.iter(){
                        let mut write_instr = &self.instr;
                        let command = format!("MEAS:CURR:DC? (@{})\n", chn);
                        let buf = command.as_bytes();
                        if let Err(_e) = write_instr.write_all(buf) {
                            let _ = send_win.emit("action_error","因未知原因掉线，请重新测试");
                            break;
                        }
                        let mut buf_reader = BufReader::new(write_instr);
                        let mut buf = String::new();
                        let _ = buf_reader.read_line(&mut buf);
                        let parsed: Result<f64, _> = buf.trim().parse();
                        let curr  =  match parsed {
                            Ok(val) => {
                                let milli_val = val * 1000.0;
                                milli_val
                            }
                            Err(_) => {
                                let _ = send_win.emit("action_error","因未知原因掉线，请重新测试");
                                break;
                            }
                        };


                        let command = format!("MEAS:VOLT:DC? (@{})\n", chn);
                        let buf = command.as_bytes();
                        if let Err(_e) = write_instr.write_all(buf) {
                            let _ = send_win.emit("action_error","因未知原因掉线，请重新测试");
                            break;
                        }
                        let mut buf_reader = BufReader::new(write_instr);
                        let mut buf = String::new();
                        let _ = buf_reader.read_line(&mut buf);
                        let parsed: Result<f64, _> = buf.trim().parse();
                        let volt = match parsed {
                            Ok(val) => {
                                let milli_val = val *1.0;
                                milli_val
                            }
                            Err(_) => {
                                let _ = send_win.emit("action_error","因未知原因掉线，请重新测试");
                                break;
                            }
                        };
                        let mut data = format!("{}:{:.3},{:.3}",chn,volt,curr);
                        if chn !="1" &&res != "" {
                            data = format!(";{}",data);
                        }
                        res+=&data;
                    }
                    let _ = send_win.emit("action_data", res);
                }
            }
        }
    }
    pub fn change_chn_open(&self, chn: String, open: String) -> Result<(), String> {
        let is_on = open;
        let cmds = format!("OUTP {},(@{})\n", is_on, chn);
        let buf = cmds.as_bytes();
        let mut write_instr = &self.instr;
        let res = write_instr.write_all(buf).map_err(|err| format!("{}", err));
        res
    }
}
