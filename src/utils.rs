//! # 工具模块
//!
//! 工具模块中包含一些简单的函数。

#![allow(deprecated)]

use std::{
    env,
    ffi::OsStr,
    fmt::Display,
    fs::{read_to_string, rename},
    io,
    path::{Path, PathBuf},
    process::{self, Command, Output, Stdio},
    sync::mpsc,
    thread,
    time::{Duration, SystemTime},
};

use ureq::Error;

use crate::constants::{CARGO_HOME, CONFIG, CONFIG_TOML, DOT_CARGO, UNC_PREFIX};

pub fn home_dir() -> PathBuf {
    env::home_dir().unwrap()
}

pub fn cargo_home() -> PathBuf {
    match env::var_os(CARGO_HOME) {
        Some(value) => PathBuf::from(value),
        None => home_dir().join(DOT_CARGO),
    }
}

pub fn cargo_config_path() -> PathBuf {
    let c = cargo_home();

    // Rust v1.39 版本中添加了对该 `.toml` 扩展的支持，并且是首选形式
    let path = c.join(CONFIG_TOML);

    // Cargo 还读取不带 `.toml` 扩展名的配置文件，例如 `~/.cargo/config`
    // 如果该文件存在，Cargo 将首先使用不带扩展名的文件
    // https://doc.rust-lang.org/cargo/reference/config.html
    let obsolete_path = c.join(CONFIG);

    if path.is_file() {
        if obsolete_path.is_file() {
            to_out(format!("检测到了两种形式的配置文件，为了避免歧义，请将 {} 文件 (不再被推荐使用) 中的内容手动合并到 {} 文件中", obsolete_path.display(), path.display()));
            process::exit(14);
        }
    } else if obsolete_path.is_file() {
        to_out(format!(
            "检测到了 {} 配置文件 (不再被推荐使用)，以后请使用 {} 配置文件",
            obsolete_path.display(),
            path.display()
        ));
        rename(obsolete_path, &path).unwrap();
    }

    path
}

pub fn get_cargo_config() -> String {
    match read_to_string(cargo_config_path()) {
        Ok(content) => content,
        Err(_) => "".to_string(),
    }
}

pub fn is_registry_name(name: Option<&String>) -> &str {
    if name.is_none() {
        to_out("请输入正确的镜像名");
        process::exit(1);
    }

    name.unwrap().as_str()
}

pub fn is_registry_addr(addr: Option<&String>) -> &str {
    if addr.is_none() {
        to_out("请输入正确的镜像地址");
        process::exit(2);
    }

    addr.unwrap().as_str()
}

pub fn is_registry_dl(dl: Option<&String>) -> &str {
    if dl.is_none() {
        to_out("请输入正确的 dl。\n  每一个镜像源都是一个 Git 存储库，而在该存储库的根目录下有一个 config.json 文件，\n  其中，dl 属性是 config.json 文件中的一个字段。");
        process::exit(3);
    }

    dl.unwrap().as_str()
}

pub fn append_end_spaces(value: &str, total_len: Option<usize>) -> String {
    let size = if total_len.is_none() {
        15
    } else {
        Option::unwrap(total_len)
    };

    let pad = if value.len() < size {
        " ".repeat(size - value.len())
    } else {
        "".to_string()
    };

    format!("{}{}", value, pad)
}

pub fn request(url: &str, is_connect_only: bool) -> Option<u128> {
    let time = SystemTime::now();

    match ureq::get(url).timeout(Duration::from_secs(5)).call() {
        Ok(res) => {
            let status = res.status();

            if status >= 300 {
                return match res.header("location") {
                    Some(v) => request(v, is_connect_only),
                    None => None,
                };
            }

            // 不管是不是 404，只要能连上主机，就成功返回
            if is_connect_only {
                return Some(time.elapsed().unwrap().as_millis());
            }

            if status >= 400 {
                return None;
            }

            Some(time.elapsed().unwrap().as_millis())
        }

        // 连接成功，但返回的状态不是预期的
        Err(Error::Status(_, _)) => {
            if is_connect_only {
                Some(time.elapsed().unwrap().as_millis())
            } else {
                None
            }
        }

        // 其他错误，例如连接失败
        Err(_) => None,
    }
}

pub fn network_delay(
    values: Vec<(String, Option<String>)>,
    sender_size: Option<usize>,
    is_connect_only: bool,
) -> Vec<(String, Option<u128>)> {
    let (tx, rx) = mpsc::channel();
    let iter = values.iter();
    let len = sender_size.unwrap_or_else(|| iter.len());
    let mut ret = vec![];

    for v in iter {
        let t = tx.clone();
        let v = v.clone();

        thread::spawn(move || {
            let date = match &v.1 {
                Some(url) => request(url, is_connect_only),
                None => None,
            };

            if t.send((v.0.to_string(), date)).is_err() {
                process::exit(0);
            }
        });
    }

    for _ in 0..len {
        ret.push(rx.recv().unwrap());
    }

    ret.sort_by(|a, b| a.1.cmp(&b.1));

    ret
}

pub fn field_eprint<T: Display>(field_name: T, field_type: &str) {
    to_out(format!(
        "{} 文件中的 {} 字段不是一个{}，请修改后重试",
        cargo_config_path().display(),
        field_name,
        field_type
    ));
}

pub fn to_out<T: Display>(message: T) {
    println!(" {}", message);
}

pub fn status_prefix(value1: &String, value2: &String) -> String {
    if value1.eq(value2) { "  * " } else { "    " }.to_string()
}

pub fn is_windows() -> bool {
    cfg!(target_os = "windows")
}

pub fn absolute_path<T: AsRef<OsStr>>(dir: &T) -> io::Result<PathBuf> {
    let mut path = Path::new(dir).canonicalize()?;
    let path_str = path.to_str().unwrap();

    // 如果是 `Windows`，并且当前路径是 `UNC` 路径
    if is_windows() && path_str.starts_with(UNC_PREFIX) {
        let path_slice = &path_str[UNC_PREFIX.len()..];

        // 路径不能超过普通 `Windows` 路径的长度
        if path_slice.len() > 260 {
            let error = io::Error::new(
                io::ErrorKind::InvalidData,
                format!("当前路径超过了 Windows 普通路径的最大长度: {}", path_str),
            );

            return Err(error);
        }

        path = PathBuf::from(path_slice);
    }

    Ok(path)
}

pub fn exec_command(command: &str, cwd: Option<&String>) -> io::Result<Output> {
    let mut program = "sh";
    let mut arg_c = "-c";

    if is_windows() {
        program = "cmd";
        arg_c = "/c";
    }

    let cwd = match cwd {
        Some(cwd) => match absolute_path(cwd) {
            Ok(path) => path,
            Err(e) => {
                return Err(e);
            }
        },
        None => env::current_dir().unwrap(),
    };

    Command::new(program)
        .current_dir(cwd)
        .args([arg_c, command])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
}

/// 获取最新的版本
pub fn get_newest_version() -> Option<String> {
    let url = "https://crates.io/api/v1/crates/crm";

    match ureq::get(url).timeout(Duration::from_secs(10)).call() {
        Ok(res) => {
            let status = res.status();

            if status >= 400 {
                return None;
            }

            match res.into_string() {
                Ok(body) => match body.find("\"newest_version\"") {
                    Some(idx) => {
                        let sub_str = &body[idx + 18..idx + 38];
                        let version = &sub_str[..sub_str.find('\"').unwrap()];

                        Some(version.to_string())
                    }

                    None => None,
                },

                Err(_) => None,
            }
        }

        Err(_) => None,
    }
}

pub fn not_command(command: &str) {
    let r = r#"
  crm best                    评估网络延迟并自动切换到最优的镜像
    crm best git              仅评估 git 镜像源
    crm best sparse           仅评估支持 sparse 协议的镜像源
    crm best git-download     仅评估能够快速下载软件包的 git 镜像源 (推荐使用)
    crm best sparse-download  仅评估能够快速下载软件包且支持 sparse 协议的镜像源 (推荐使用)
  crm current                 获取当前所使用的镜像
  crm default                 恢复为官方默认镜像
  crm install [args]          使用官方镜像执行 "cargo install"
  crm list                    从镜像配置文件中获取镜像列表
  crm publish [args]          使用官方镜像执行 "cargo publish"
  crm remove <name>           在镜像配置文件中删除镜像
  crm save <name> <addr> <dl> 在镜像配置文件中添加/更新镜像
  crm test [name]             下载测试包以评估网络延迟
  crm update [args]           使用官方镜像执行 "cargo update"
  crm use <name>              切换为要使用的镜像
  crm version                 查看当前版本
  crm check-update            检测版本更新
"#;

    to_out(format!(
        "{} 命令无效。参考:\n{}\nHome：<https://github.com/wtklbm/crm>\n",
        command, r
    ));
    process::exit(4);
}
