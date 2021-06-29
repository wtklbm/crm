//! # 工具模块
//!
//! 工具模块中包含一些简单的函数。

#![allow(deprecated)]

use std::{env, fs::read_to_string, path::PathBuf, process};

use crate::constants::{CARGO, CARGO_HOME, CONFIG};

pub fn home_dir() -> PathBuf {
    env::home_dir().unwrap()
}

pub fn cargo_home() -> PathBuf {
    match env::var_os(CARGO_HOME) {
        Some(value) => PathBuf::from(value),
        None => home_dir().clone().join(CARGO),
    }
}

pub fn cargo_config_path() -> PathBuf {
    let mut c = cargo_home();
    c.push(CONFIG);

    c
}

pub fn get_cargo_config() -> String {
    match read_to_string(cargo_config_path()) {
        Ok(content) => content,
        Err(_) => String::new(),
    }
}

pub fn is_registry_name(name: Option<&String>) -> &str {
    if name.is_none() {
        println!("请输入正确的镜像名");
        process::exit(0);
    }

    name.unwrap().as_str()
}

pub fn is_registry_addr(addr: Option<&String>) -> &str {
    if addr.is_none() {
        println!("请输入正确的镜像地址");
        process::exit(0);
    }

    addr.unwrap().as_str()
}

pub fn is_registry_dl(dl: Option<&String>) -> &str {
    if dl.is_none() {
        println!("请输入正确的 dl。\n  每一个镜像源都是一个 Git 存储库，而在该存储库的根目录下有一个 config.json 文件，\n  其中，dl 属性是 config.json 文件中的一个字段。");
        process::exit(0);
    }

    dl.unwrap().as_str()
}

pub fn not_command(command: &str) {
    let r = r#"
  crm best                    评估网络延迟并使用最优的镜像
  crm current                 获取当前所使用的镜像
  crm default                 恢复为默认的镜像
  crm list                    从镜像配置文件中获取镜像列表
  crm remove <name>           在镜像配置文件中删除镜像
  crm save <name> <addr> <dl> 在镜像配置文件中添加/更新镜像
  crm test [name]             下载测试包以评估网络延迟
  crm use <name>              切换为要使用的镜像
"#;
    println!("{} 命令无效。参考:\n{}", command, r.trim_end());
    process::exit(0);
}
