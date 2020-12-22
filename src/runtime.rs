//! # 运行时
//!
//! `runtime` 是一个处理程序运行时配置的模块，通过它，我们可以通过执行的命令来更改 `.crmrc` 文件。
//! 而 `.crmrc` 文件里面存储的是关于 `Cargo` 配置的相关信息。

#![allow(unused)]

use std::{
    collections::HashMap,
    fs::{self, read_to_string, write},
    path::PathBuf,
};

use crate::{
    constants::{CRMRC, CRMRC_FILE, PUBLIC_RC},
    util::home_dir,
};

/// 运行时配置
#[derive(Debug)]
pub struct RuntimeConfig {
    /// 运行时配置的存放路径
    path: PathBuf,

    /// 运行时配置的内容
    data: String,

    /// 运行时配置中读取到的镜像映射表
    pub map: HashMap<String, String>,
}

impl RuntimeConfig {
    /// 创建运行时配置对象
    pub fn new() -> Self {
        let mut map = HashMap::new();
        let rc_path = home_dir().join(CRMRC);
        let data = match read_to_string(&rc_path) {
            Ok(content) => content,
            Err(_) => {
                write(&rc_path, CRMRC_FILE.trim_start()).unwrap();
                read_to_string(&rc_path).unwrap()
            }
        };

        data.lines().for_each(|line| {
            let line = line.trim();

            // 默认以 `#` 号开头的行为注释，而注释行不需要解析
            if line.starts_with("#") || line.is_empty() {
                return;
            }

            let (name, addr) = line.split_once("=").unwrap();
            map.insert(name.trim().to_string(), addr.trim().to_string());
        });

        RuntimeConfig {
            data,
            path: rc_path,
            map,
        }
    }

    /// 将运行时配置中的镜像列表转换为字符串
    pub fn to_string(&self, mut sep: Option<&str>) -> String {
        let sep = if let None = sep { " = " } else { sep.unwrap() };

        self.map.iter().fold(String::new(), |mut memo, (k, v)| {
            memo.push_str(&format! {"{}{}{}\n", k,sep, v });
            memo
        })
    }

    /// 将运行时配置中的镜像列表名转换为字符串
    pub fn to_key_string(&self) -> String {
        self.map
            .keys()
            .map(|key| format!("  - {}", key))
            .collect::<Vec<String>>()
            .join("\n")
    }

    /// 将运行时配置写入到文件中
    pub fn write(&self) {
        fs::write(&self.path, self.to_string(None)).unwrap();
    }

    /// 获取运行时配置中的某一个属性
    pub fn get(&self, registry_name: &str) -> Option<&String> {
        self.map.get(registry_name)
    }

    /// 将属性添加到运行时配置中
    pub fn add(&mut self, registry_name: &str, registry_addr: &str) {
        self.map
            .entry(registry_name.to_string())
            .or_insert(registry_addr.to_string());
    }

    /// 更新运行时配置中的属性
    pub fn update(&mut self, registry_name: &str, registry_addr: &str) {
        self.map
            .insert(registry_name.to_string(), registry_addr.to_string());
    }

    /// 删除运行时配置中的属性
    pub fn remove(&mut self, registry_name: &str) {
        self.map.remove(registry_name);
    }
}
