use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
pub struct UseCaseDataFile {
    file: File,
}

impl UseCaseDataFile {
    pub fn new(path: PathBuf) -> Result<Self, String> {
        let file_path = path.display().to_string();
        let file = File::create(file_path);
        match file {
            Ok(file) => Ok(UseCaseDataFile { file }),
            Err(err) => Err(format!("open file or create file error:{}", err)),
        }
    }
    pub fn write(&self, data: String) {
        let mut writer_data = BufWriter::new(&self.file);

        let is_ok = writeln!(writer_data, "{}", data);
        let is_writer = writer_data.flush();
        match is_writer {
            Ok(_) => {
                println!("write data success");
            }
            Err(err) => {
                println!("write data error:{}", err);
            }
        }
    }
}
