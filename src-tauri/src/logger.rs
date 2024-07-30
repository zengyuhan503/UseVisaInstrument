use chrono::Local;
use log::LevelFilter;
use simplelog::*;
use std::path::PathBuf;
use std::{
    env,
    fs:: OpenOptions,
};
use time::UtcOffset;

pub fn get_executable_path() -> PathBuf {
    env::current_exe().expect("无法获取可执行文件路径")
}

pub fn init_logging() {
    // 确保 log 目录存在
    let exe_path = get_executable_path();
    let log_dir = exe_path.parent().unwrap().join("log");
    // 根据当前日期生成日志文件名称
    let date = Local::now().format("%Y-%m-%d").to_string();
    let log_file_path = log_dir.join(format!("{}.log", date));
    let _log_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(log_file_path);

    let offset = UtcOffset::from_hms(8, 0, 0).unwrap();
    // 创建或打开日志文件
    match _log_file {
        Ok(_file) => WriteLogger::init(
            LevelFilter::Info,
            ConfigBuilder::new()
                .set_time_offset(offset)
                .set_time_format_custom(format_description!(
                    "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"
                ))
                .set_target_level(LevelFilter::Off)
                .set_thread_level(LevelFilter::Off)
                .build(),
            _file,
        )
        .expect("错误"),
        Err(err) => {
            println!("error {:?}", err);
        }
    }
}
