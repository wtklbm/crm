//! # 对镜像进行操作
//!
//! 该模块用于操作镜像。包括简单的增删改查操作。

use crate::runtime::RuntimeConfig;
use crate::{
    cargo::CargoConfig,
    util::{is_registry_addr, is_registry_name},
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

        if let Err(name) = self.cargo.use_registry(name, self.rc.map.get(name)) {
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

        self.rc.remove(name);
        self.rc.write();
    }

    /// 添加镜像
    pub fn add(&mut self, name: Option<&String>, addr: Option<&String>) {
        let name = is_registry_name(name).trim();
        let addr = is_registry_addr(addr).trim();

        self.rc.add(name, addr);
        self.rc.write()
    }

    /// 更新镜像
    pub fn update(&mut self, name: Option<&String>, addr: Option<&String>) {
        let name = is_registry_name(name).trim();
        let addr = is_registry_addr(addr).trim();

        self.rc.update(name, addr);
        self.rc.write();
    }

    /// 获取镜像列表
    pub fn list(&self) -> String {
        self.rc.to_string(Some(": "))
    }

    /// 获取当前正在使用的镜像
    pub fn current(&self) -> (String, Option<String>) {
        let (name, addr) = CargoConfig::new().current();
        let addr = match addr {
            Some(addr) => Some(addr),
            None => match self.rc.get(&name) {
                Some(addr) => Some(addr.to_string()),
                None => None,
            },
        };

        (name, addr)
    }

    /// 恢复为默认镜像
    pub fn default(&self) {
        let (name, addr) = self.current();

        match addr {
            Some(addr) => println!("{}: {}", name, addr),
            None => println!("{}", name),
        };
    }
}
