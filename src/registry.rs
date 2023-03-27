//! # 对镜像进行操作
//!
//! 该模块用于操作镜像。包括简单的增删改查操作。

use std::process;

use crate::{
    cargo::CargoConfig,
    constants::{APP_NAME, APP_VERSION, CARGO, RUST_LANG},
    runtime::RuntimeConfig,
    utils::{
        append_end_spaces, exec_command, is_registry_addr, is_registry_dl, is_registry_name,
        network_delay, status_prefix, to_out,
    },
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

        // 收集需要添加 `[registries.xxx]` 属性的镜像元祖数组
        let remaining_registries = self.rc.to_tuples(None);

        if let Err(name) = self
            .cargo
            .use_registry(name, self.rc.get(name), remaining_registries)
        {
            let keys = self.rc.to_key_string();

            if keys.is_empty() {
                return to_out(format!(
                    "没有找到 {} 镜像，配置中的镜像列表为空，请用 \"crm save\" 添加镜像后重试",
                    name,
                ));
            }

            to_out(format!("没有找到 {} 镜像，可选的镜像是:\n{}", name, keys));
        };

        self.cargo.make();
    }

    /// 删除镜像
    pub fn remove(&mut self, name: Option<&String>) {
        let name = is_registry_name(name).trim();

        if self.rc.get_default(name).is_some() {
            to_out("请不要删除内置镜像");
            process::exit(11);
        }

        if self.rc.get_extend(name).is_none() {
            to_out(format!("删除失败，{} 镜像不存在", name));
            process::exit(12);
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
    pub fn list(&self, current: &String) -> String {
        self.rc.to_string(current, Some("- "))
    }

    /// 获取当前正在使用的镜像
    pub fn current(&self) -> (String, Option<String>) {
        let (name, addr) = CargoConfig::new().current();
        let addr = match addr {
            Some(addr) => Some(addr),
            None => self.rc.get(&name).map(|addr| addr.registry.clone()),
        };

        (name, addr)
    }

    /// 恢复为默认镜像
    pub fn default(&mut self) {
        self.select(Some(&RUST_LANG.to_string()));
    }

    /// 评估网络延迟并自动切换到最优的镜像
    pub fn best(&mut self) {
        let tested = self.test_download_status(None, Some(1));
        let found = tested.iter().find(|v| v.1.is_some());

        if found.is_none() {
            return to_out("没有可切换的镜像源");
        }

        let registry_name = &found.unwrap().0;

        self.select(Some(registry_name));
        to_out(format!("已切换到 {} 镜像源", registry_name));
    }

    /// 将 `dl` 转换为 `url`
    fn to_download_url(&self, name: &str) -> Option<String> {
        match self.rc.get(name) {
            Some(rd) => {
                let dl = rd.dl.clone();
                let url = if !dl.ends_with("/api/v1/crates") {
                    dl.replace("{crate}", APP_NAME)
                        .replace("{version}", APP_VERSION)
                } else {
                    format!("{}/{}/{}/download", dl, APP_NAME, APP_VERSION)
                };

                Some(url)
            }
            None => None,
        }
    }

    /// 测试镜像源状态
    /// NOTE 仅 `git` 镜像源支持下载状态测试
    fn test_download_status(
        &self,
        name: Option<&String>,
        sender_size: Option<usize>,
    ) -> Vec<(String, Option<u128>)> {
        let urls = match name {
            Some(name) => {
                if self.rc.get(name).is_none() {
                    to_out(format!("测试失败，{} 镜像不存在", name));
                    process::exit(13);
                }

                vec![(name.to_string(), self.to_download_url(name))]
            }
            None => self
                .rc
                .registry_names()
                .iter()
                .filter(|name| !name.ends_with("-sparse"))
                .map(|name| (name.to_string(), self.to_download_url(name)))
                .collect(),
        };

        network_delay(urls, sender_size, false)
    }

    /// 将 `dl` 转换为 `url`
    fn to_connected_url(&self, name: &str) -> Option<String> {
        match self.rc.get(name) {
            Some(rd) => {
                let registry = rd.registry.clone();

                if registry.starts_with("git:") {
                    return None;
                }

                Some(registry.replace("sparse+", ""))
            }
            None => None,
        }
    }

    fn test_connected_status(
        &self,
        name: Option<&String>,
        sender_size: Option<usize>,
    ) -> Vec<(String, Option<u128>)> {
        let urls = match name {
            Some(name) => {
                if self.rc.get(name).is_none() {
                    to_out(format!("测试失败，{} 镜像不存在", name));
                    process::exit(13);
                }

                vec![(name.to_string(), self.to_connected_url(name))]
            }
            None => self
                .rc
                .registry_names()
                .iter()
                .map(|name| (name.to_string(), self.to_connected_url(name)))
                .collect(),
        };

        network_delay(urls, sender_size, true)
    }

    /// 测试镜像源延迟
    pub fn test(&self, current: &String, name: Option<&String>) {
        let connected_status: Vec<String> = self
            .test_connected_status(name, None)
            .iter()
            .map(|(name, status)| {
                let prefix = status_prefix(name, current);
                let name = append_end_spaces(name, None);
                let status = match status {
                    Some(s) => format!("{} ms", s),
                    None => "failed".to_string(),
                };

                format!("{}{} -- {}", prefix, name, status)
            })
            .collect();

        println!("网络连接延迟:\n{}\n", connected_status.join("\n"));

        let download_status: Vec<String> = self
            .test_download_status(name, None)
            .iter()
            .map(|(name, status)| {
                let prefix = status_prefix(name, current);
                let name = append_end_spaces(name, None);
                let status = match status {
                    Some(s) => format!("{} ms", s),
                    None => "failed".to_string(),
                };

                format!("{}{} -- {}", prefix, name, status)
            })
            .collect();

        println!("软件包下载延迟:\n{}", download_status.join("\n"));
    }

    /// 使用官方镜像源执行命令
    fn exec(&mut self, command: &str) {
        let (registry_name, _) = self.current();
        let is_default_registry = registry_name.eq(RUST_LANG);

        if !is_default_registry {
            self.default();
        }

        if let Err(e) = exec_command(command, None) {
            to_out(e);
        };

        if !is_default_registry {
            self.select(Some(&registry_name));
        }
    }

    /// 使用官方镜像执行 `cargo publish`
    pub fn publish(&mut self, args: String) {
        self.exec(&format!("{} publish {}", CARGO, args.trim()));
    }

    /// 使用官方镜像执行 `cargo update`
    pub fn update(&mut self, args: String) {
        self.exec(&format!("{} update {}", CARGO, args.trim()));
    }

    /// 使用官方镜像执行 `cargo install`
    pub fn install(&mut self, args: String) {
        let args = args.trim();
        let args = if args.is_empty() { "--help" } else { args };

        self.exec(&format!("{} install {}", CARGO, args));
    }
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}
