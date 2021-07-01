//! # 工具模块
//!
//! 工具模块中包含一些简单的函数。

#![allow(deprecated)]

use std::{env, fmt::Display, fs::read_to_string, path::PathBuf, process, time::Duration};

use crate::constants::{CARGO, CARGO_CONFIG_PATH, CARGO_HOME, CONFIG};

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
        Err(_) => "".to_string(),
    }
}

pub fn is_registry_name(name: Option<&String>) -> &str {
    if name.is_none() {
        error_print("请输入正确的镜像名");
        process::exit(-1);
    }

    name.unwrap().as_str()
}

pub fn is_registry_addr(addr: Option<&String>) -> &str {
    if addr.is_none() {
        error_print("请输入正确的镜像地址");
        process::exit(-1);
    }

    addr.unwrap().as_str()
}

pub fn is_registry_dl(dl: Option<&String>) -> &str {
    if dl.is_none() {
        error_print("请输入正确的 dl。\n  每一个镜像源都是一个 Git 存储库，而在该存储库的根目录下有一个 config.json 文件，\n  其中，dl 属性是 config.json 文件中的一个字段。");
        process::exit(-1);
    }

    dl.unwrap().as_str()
}

pub fn append_end_spaces(value: &str, total_len: Option<usize>) -> String {
    let size = if total_len.is_none() {
        15
    } else {
        total_len.unwrap()
    };

    let pad = if value.len() < size {
        " ".repeat(size - value.len())
    } else {
        "".to_string()
    };

    format!("{}{}", value, pad)
}

pub fn request(url: &String) -> bool {
    let time = Duration::from_secs(10);

    if let Ok(response) = ureq::get(url).timeout(time).call() {
        let status = response.status();

        if status >= 400 {
            return false;
        }

        if status >= 300 {
            return match response.header("location") {
                Some(v) => request(&v.to_string()),
                None => false,
            };
        }

        return true;
    }

    false
}

pub fn field_eprint(field_name: &str, field_type: &str) {
    error_print(format!(
        "{} 文件中的 {} 字段不是一个{}，请修改后重试",
        CARGO_CONFIG_PATH, field_name, field_type
    ));
}

pub fn error_print<T: Display>(message: T) {
    eprint!(" {}", message);
}

pub fn not_command(command: &str) {
    let r = r#"
  crm best                    评估网络延迟并自动切换到最优的镜像
  crm current                 获取当前所使用的镜像
  crm default                 恢复为默认的镜像
  crm list                    从镜像配置文件中获取镜像列表
  crm remove <name>           在镜像配置文件中删除镜像
  crm save <name> <addr> <dl> 在镜像配置文件中添加/更新镜像
  crm test [name]             下载测试包以评估网络延迟
  crm use <name>              切换为要使用的镜像
"#;

    error_print(format!("{} 命令无效。参考:\n{}", command, r.trim_end()));
    process::exit(-1);
}
