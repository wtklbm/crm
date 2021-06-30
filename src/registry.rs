//! # 对镜像进行操作
//!
//! 该模块用于操作镜像。包括简单的增删改查操作。

use std::{process, time::SystemTime};

use crate::{
    cargo::CargoConfig,
    constants::{APP_NAME, APP_VERSION, RUST_LANG},
    runtime::RuntimeConfig,
    util::{append_end_spaces, is_registry_addr, is_registry_dl, is_registry_name, request},
};

/// 镜像对象
pub struct Registry {
    /// 运行时配置
    rc: RuntimeConfig,

    /// `Cargo` 配置
    cargo: CargoConfig,
}

impl Registry {
    /// 创建镜像对象
    pub fn new() -> Self {
        Registry {
            rc: RuntimeConfig::new(),
            cargo: CargoConfig::new(),
        }
    }

    /// 切换镜像
    pub fn select(&mut self, name: Option<&String>) {
        let name = is_registry_name(name).trim();

        if let Err(name) = self.cargo.use_registry(name, self.rc.get(name)) {
            let keys = self.rc.to_key_string();

            if keys.is_empty() {
                return println!(
                    "没有找到 \"{}\" 镜像，配置中的镜像列表为空，请用 \"crm add\" 添加镜像后重试",
                    name
                );
            }

            println!("没有找到 \"{}\" 镜像，可选的镜像是:\n{}", name, keys);
        };

        self.cargo.make();
    }

    /// 删除镜像
    pub fn remove(&mut self, name: Option<&String>) {
        let name = is_registry_name(name).trim();

        if let Some(_) = self.rc.get_default(name) {
            println!("请不要删除内置镜像");
            process::exit(-1);
        }

        if let None = self.rc.get_extend(name) {
            println!("删除失败，{:?} 镜像不存在", name);
            process::exit(-1);
        }

        self.rc.remove(name);
        self.rc.write();
    }

    /// 添加/更新镜像
    pub fn save(&mut self, name: Option<&String>, addr: Option<&String>, dl: Option<&String>) {
        let name = is_registry_name(name).trim();
        let addr = is_registry_addr(addr).trim();
        let dl = is_registry_dl(dl).trim();

        self.rc.save(name, addr, dl);
        self.rc.write();
    }

    /// 获取镜像列表
    pub fn list(&self) -> String {
        self.rc.to_string(Some("- "))
    }

    /// 获取当前正在使用的镜像
    pub fn current(&self) -> (String, Option<String>) {
        let (name, addr) = CargoConfig::new().current();
        let addr = match addr {
            Some(addr) => Some(addr),
            None => match self.rc.get(&name) {
                Some(addr) => Some(addr.registry.clone()),
                None => None,
            },
        };

        (name, addr)
    }

    /// 恢复为默认镜像
    pub fn default(&mut self) {
        self.select(Some(&RUST_LANG.to_string()));
    }

    /// 评估网络延迟并自动切换到最优的镜像
    pub fn best(&mut self) {
        let tested = self.test_status(None);
        let found = tested.iter().find(|v| v.1.is_some());

        if found.is_none() {
            return println!("没有可切换的镜像源");
        }

        let registry_name = &found.unwrap().0;

        self.select(Some(registry_name));
        println!("已切换到 {} 镜像源", registry_name);
    }

    /// 测试指定的镜像源延迟
    fn test_by_name(&self, name: &str) -> (String, Option<u128>) {
        if let Some(rd) = self.rc.get(name) {
            let dl = rd.dl.clone();

            // 拼接链接地址
            let url = if !dl.ends_with("/api/v1/crates") {
                dl.replace("{crate}", APP_NAME)
                    .replace("{version}", APP_VERSION)
            } else {
                format!("{}/{}/{}/download", dl, APP_NAME, APP_VERSION)
            };

            // 获取当前的时间
            let time_now = SystemTime::now();

            // 发起请求
            if request(&url) {
                // 如果请求返回成功计算总延迟
                let millis = time_now.elapsed().unwrap().as_millis();

                return (name.to_string(), Some(millis));
            }

            // 请求失败
            return (name.to_string(), None);
        }

        return (name.to_string(), None);
    }

    /// 测试镜像源状态
    fn test_status(&self, name: Option<&String>) -> Vec<(String, Option<u128>)> {
        let mut status: Vec<(String, Option<u128>)> = Vec::new();

        // 如果没有传递镜像名
        if let None = name {
            // 进行完整性测试
            self.rc.registry_names().iter().for_each(|name| {
                status.push(self.test_by_name(name));
            });
        } else {
            // 仅测试当前镜像
            let name = is_registry_name(name).trim();
            status.push(self.test_by_name(name));
        }

        // 对毫秒数进行排序
        status.sort_by(|a, b| a.1.cmp(&b.1));

        status
    }

    /// 测试镜像源延迟
    pub fn test(&self, name: Option<&String>) {
        // 拼接状态字符串
        let status: Vec<String> = self
            .test_status(name)
            .iter()
            .map(|(name, status)| {
                let new_name = append_end_spaces(name, None);

                if let None = status {
                    format!("  {} -- failed", new_name)
                } else {
                    format!("  {} -- {} ms", new_name, status.unwrap())
                }
            })
            .collect();

        println!("{}", status.join("\n"));
    }
}
