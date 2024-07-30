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
    pub fn new(instr_ip: String) -> Result<Self, String> {
        log::info!("初始化DEFAULTR 实例");
        let rm = RESOURCE_DEFAULTRM.lock().map_err(|e| format!("{:?}", e))?;
        log::info!("创建CString 实例");
        let ip_s = format!("TCPIP0::{}::inst0::INSTR", instr_ip);
        let expr = CString::new(ip_s)
            .map_err(|err| format!("没有找到相关资源：{:?}", err))?
            .into();

        let rsc = rm
            .find_res(&expr)
            .map_err(|op| format!("未找到相关的资源{:?}", op))?;

        let instr = rm
            .open(&rsc, AccessMode::NO_LOCK, TIMEOUT_IMMEDIATE)
            .map_err(|op| format!("{:?}", op))?;
        Ok(Self { instr })
    }
    pub fn find_all_resources() -> Result<Vec<VisaString>, String> {
        let rm = RESOURCE_DEFAULTRM.lock().map_err(|e| format!("{:?}", e))?;
        let expr = CString::new("?*INSTR").map_err(|err| format!("没有找到相关资源：{:?}", err))?;
        let mut visa_list: Vec<VisaString> = Vec::new();
        let list = rm.find_res_list(&expr.into());
        match list {
            Ok(list) => {
                for item in list {
                    match item {
                        Ok(vs) => {
                            visa_list.push(vs);
                        }
                        Err(err) => {
                            println!("error:{:?}", err)
                        }
                    }
                }
            }
            Err(err) => {
                return Err(format!("未找到相关的资源{:?}", err));
            }
        }
        Ok(visa_list)
    }
    pub async fn close_all(&mut self) -> Result<(), String> {
        let cmds = format!("OUTP OFF,(@1:3)\n");
        let cmd = cmds.as_bytes();
        let mut write_instr = &self.instr;
        let res = write_instr.write_all(cmd).map_err(|err| format!("{}", err));
        let _ = self.instr.clear();
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
