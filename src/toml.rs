//!  编译 `toml` 的模块
//!
//! 该模块用来解析 `toml` 文件，可以对 `toml` 文件进行更改，当更改完成之后可以再序列化为 `toml` 字符串。

use std::{
    fs::{create_dir_all, write},
    path::Path,
    process,
};

use toml_edit::{DocumentMut, Table, TomlError};

use crate::utils::to_out;

#[derive(Debug)]
pub struct Toml {
    /// 文档
    pub doc: DocumentMut,
}

impl Toml {
    /// 解析 `toml` 字符串
    pub fn parse(input: &str) -> Result<Toml, TomlError> {
        match input.parse::<DocumentMut>() {
            Ok(doc) => Ok(Toml { doc }),
            Err(e) => Err(e),
        }
    }

    /// 转换为不可变表
    pub fn table(&self) -> &Table {
        self.doc.as_table()
    }

    /// 转换为可变表
    pub fn table_mut(&mut self) -> &mut Table {
        self.doc.as_table_mut()
    }

    /// 转换为字符串
    pub fn toml_string(&self) -> String {
        self.doc.to_string().trim().to_string()
    }

    /// 写入到文件中
    pub fn write<P: AsRef<Path>>(&self, path: P) {
        let parent = path.as_ref().parent().unwrap();

        if !parent.is_dir() {
            create_dir_all(parent).unwrap();
        }

        if let Err(e) = write(path, self.toml_string()) {
            to_out(format!("写入文件失败:\n  {}", e));
            process::exit(18);
        }
    }
}
