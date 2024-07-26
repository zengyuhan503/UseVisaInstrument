// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use tokio::sync::{mpsc, Mutex, RwLock};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tauri::Manager;

mod use_visa_instrument;
use use_visa_instrument::VisaInstrument;

mod logger;
use logger::init_logging;

mod use_case_data_file;

#[tauri::command]
fn find_all_visa_instrument() -> Result<Vec<String>, String> {
    log::info!("初始化，获取全局的资源");
    let find = use_visa_instrument::VisaInstrument::find_all_resources();
    match find {
        Ok(list)=>{
            let list_str = list.iter().map(|s| s.to_string()).collect::<Vec<String>>();
            Ok(list_str)
        },
        Err(err)=>Err(err.to_string())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ActionParams {
    name: String,
    item1: String,
    item2: String,
}

// 全局或高级作用域中维护最新的 Sender
struct Control {
    stop_tx: Option<mpsc::Sender<()>>,
}

impl Control {
    fn new() -> Self {
        Control { stop_tx: None }
    }
}

lazy_static! {
    static ref VISA_INSTRUMENT: Arc<RwLock<Option<VisaInstrument>>> = Arc::new(RwLock::new(None));
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_win = app.get_window("main").unwrap();
            init_logging();
            let listen_win = app_win.clone();
            let control = Arc::new(Mutex::new(Control::new()));
            listen_win.listen("action_req", move |event| {
                let control = control.clone();
                let emit_win = app_win.clone();
                tokio::spawn(async move {
                    if let Some(data) = event.clone().payload() {
                        let params: ActionParams = serde_json::from_str(data).expect("No json");
                        log::info!("params:{:?}", params);
                        let name = params.name.clone();
                        if name.as_str() == "stop_read" {
                            let mut ctrl = control.lock().await;
                            if let Some(stop_tx) = ctrl.stop_tx.take() {
                                println!("Sending stop signal...");
                                log::info!("Sending stop signal...,关闭数据循环");
                                stop_tx.send(()).await.expect("Failed to send stop signal");
                                let _ = emit_win.emit("action_res", "ok");
                                println!("Stop signal sent.");
                            } else {
                                println!("No stop signal available.");
                            }
                        };

                        log::info!("初始化visa_instrument");

                        log::info!(" match name");
                        match name.as_str() {
                            "powerOn" => {
                                let instr_ip = params.item2.clone();

                                log::info!("开启设备时 获取chn");

                                log::info!("初始化instr 实例");
                                let instr = VisaInstrument::new( instr_ip);
                                match instr {
                                    Ok(instr) => {
                                        let mut visa_instrument = VISA_INSTRUMENT.write().await;
                                        *visa_instrument = Some(instr);
                                        let res = "ok：已经连接设备，请进行测试";
                                        let _emit = emit_win.emit("action_res", res);
                                    }
                                    Err(err) => {
                                        log::error!("error：{}", err);
                                        let res = format!("error：{}", err);
                                        let _emit = emit_win.emit("action_res", res);
                                    }
                                }
                            }
                            "powerOff" => {
                                let mut visa_instrument = VISA_INSTRUMENT.write().await;
                                if let Some(ref mut instrs) = *visa_instrument {
                                    let res = instrs.close_all().await;
                                    match res {
                                        Ok(_) => {
                                            let res = "ok：已经关闭通道连接";
                                            let _emit = emit_win.emit("action_res", res);
                                        }
                                        Err(err) => {
                                            log::error!("error：{}", err);
                                            let res = format!("error：{}", err);
                                            let _emit = emit_win.emit("action_res", res);
                                        }
                                    }
                                }
                            }
                            "set_config" => {
                                let item1 = params.item1.clone();
                                let mut visa_instrument = VISA_INSTRUMENT.write().await;
                                if let Some(ref mut instrs) = *visa_instrument {
                                    let res = instrs.set_config(item1);
                                    match res {
                                        Ok(_) => {
                                            let res = "ok：参数设置成功";
                                            let _emit = emit_win.emit("action_res", res);
                                        }
                                        Err(err) => {
                                            log::error!("error：{}", err);
                                            let res = format!("error：{}", err);
                                            let _emit = emit_win.emit("action_res", res);
                                        }
                                    }
                                }
                            }
                            "start_read" => {
                                let items = params.item1;
                                let split_time = params.item2;
                                let (stop_tx, stop_rx) = mpsc::channel::<()>(1);
                                let mut ctrl = control.lock().await;
                                ctrl.stop_tx = Some(stop_tx);
                                drop(ctrl); // Drop lock before awaiting

                                let send_win = emit_win.clone();
                                let visa_instrument = VISA_INSTRUMENT.read().await;
                                if let Some(ref instrs) = *visa_instrument {
                                    println!("Starting read current data...");
                                    instrs
                                        .read_current_data(send_win, stop_rx, items, split_time)
                                        .await;
                                }
                            }
                            "switch_chn" => {
                                let item = params.item1;
                                let is_open = params.item2;
                                let chn = item.replace("o", "");
                                let visa_instrument = VISA_INSTRUMENT.read().await;
                                if let Some(ref instrs) = *visa_instrument {
                                    let res = instrs.change_chn_open(chn, is_open);
                                    let _ = emit_win.emit("action_req", res);
                                }
                            }
                            "read_line_data" => {}
                            "save_case_data" => {
                                // let data =params.item1;
                            }
                            _ => {
                                println!("Unknown event")
                            }
                        }
                    };
                });
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![find_all_visa_instrument])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
// C:\Program Files (x86)\IVI Foundation\VISA\WinNT\Bin
// C:\Program Files\IVI Foundation\VISA\Win64\Bin\
// C:\Program Files (x86)\IVI Foundation\VISA\WinNT\Bin\