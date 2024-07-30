use std::{
    env,
    fs::{File, OpenOptions},
    io::{self, Read, Seek, SeekFrom, Write},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

// 定义数据结构
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Legend {
    data: Vec<String>,
    textStyle: TextStyle,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextStyle {
    color: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Series {
    name: String,
    data: Vec<f64>,
    yAxisIndex: u32,
    tooltip: Tooltip,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tooltip {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Content {
    legend: Legend,
    series: Vec<Series>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TempData {
    name: String,
    content: Content,
}

#[derive(Debug, Clone)]
pub struct TempFile {
    path: PathBuf,
}

impl TempFile {
    pub fn new() -> Self {
        let mut temp_dir = env::temp_dir();
        temp_dir.push("visa_data_temp.json");
        TempFile { path: temp_dir }
    }

    pub fn initialize(&self) -> io::Result<()> {
        // 检查文件是否存在，如果不存在则创建并初始化
        if !self.path.exists() {
            let initial_data: Vec<TempData> = vec![]; // 初始化为一个空的 Data 向量
            let json_data = serde_json::to_string(&initial_data)?;
            let mut file = File::create(&self.path)?;
            file.write_all(json_data.as_bytes())?;
            file.flush()?;
        }
        Ok(())
    }

    pub fn write(&self, data: &TempData) -> io::Result<()> {
        // 读取现有内容
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&self.path)?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        // 反序列化现有内容
        let mut data_vec: Vec<TempData> = if contents.trim().is_empty() {
            vec![]
        } else {
            serde_json::from_str(&contents)?
        };

        // 添加新的数据
        data_vec.push(data.clone());

        // 序列化并写回文件
        let json_data = serde_json::to_string(&data_vec)?;
        file.set_len(0)?; // 清空文件
        file.seek(SeekFrom::Start(0))?; // 将文件指针移动到文件开头
        file.write_all(json_data.as_bytes())?;
        file.flush()?;

        Ok(())
    }
    pub fn read(&self) -> io::Result<Vec<TempData>> {
        let mut file = File::open(&self.path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let data: Vec<TempData> = serde_json::from_str(&contents)?;
        Ok(data)
    }
    pub fn delete(&self, index: usize) -> io::Result<()> {
        // 读取现有内容
        let mut file = OpenOptions::new().read(true).write(true).open(&self.path)?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // 反序列化现有内容
        let mut data_vec: Vec<TempData> = if contents.trim().is_empty() {
            vec![]
        } else {
            serde_json::from_str(&contents)?
        };

        // 检查索引是否有效
        if index < data_vec.len() {
            // 删除指定索引的数据
            data_vec.remove(index);

            // 序列化并写回文件
            let json_data = serde_json::to_string(&data_vec)?;
            file.set_len(0)?; // 清空文件
            file.seek(SeekFrom::Start(0))?; // 将文件指针移动到文件开头
            file.write_all(json_data.as_bytes())?;
            file.flush()?;
        } else {
            println!("Invalid index: {}", index);
        }

        Ok(())
    }
}
