//! # 处理 `Cargo` 配置
//!
//! 该模块会解析 `Cargo` 配置，并将其反序列化为对象的形式，修改完成后再序列化为相应的文件。
//! `CargoConfig` 是一个操作 `Cargo` 配置文件的对象，有了它一切都好办了。

use std::{cell::RefCell, fs};

use toml::{
    self,
    map::Map,
    Value::{self, String as VString, Table},
};

use crate::{
    constants::{CRATES_IO, REGISTRY, REPLACE_WITH, RUST_LANG, SOURCE},
    util::{cargo_config_path, get_cargo_config},
};

/// `Cargo` 配置对象
pub struct CargoConfig {
    /// 配置对象中的数据，它是一个经过反序列化的对象
    data: RefCell<Value>,
}

impl CargoConfig {
    /// 创建配置对象
    pub fn new() -> Self {
        let toml = get_cargo_config();

        // 如果文件是空的
        if toml.trim().is_empty() {
            let mut m = Map::new();
            m.insert(SOURCE.to_string(), Table(Map::new()));

            return CargoConfig {
                data: RefCell::new(Table(m)),
            };
        }

        let mut config = toml::from_str::<Value>(&toml).unwrap();

        // 如果配置文件中的 `source` 属性不是 `Table`，则创建一个空的 `Table` 赋值给 `source`。
        if config.get(SOURCE).is_none() || !config.get(SOURCE).unwrap().is_table() {
            config
                .as_table_mut()
                .unwrap()
                .entry(SOURCE)
                .or_insert(Table(Map::new()));
        }

        CargoConfig {
            data: RefCell::new(config),
        }
    }

    /// 将 `Cargo` 配置写入到文件中
    pub fn make(&self) {
        let data = self.data.borrow();
        let file_content = toml::to_string(&(*data)).unwrap();

        fs::write(cargo_config_path(), file_content).unwrap();
    }

    /// 如果 `Cargo` 配置文件中不包含 `[source.crates-io]` 属性，则为 `Cargo` 配置自动填充。
    fn fill_crates_io(&self) {
        if self
            .data
            .borrow()
            .get(SOURCE)
            .unwrap()
            .get(CRATES_IO)
            .is_none()
        {
            self.data
                .borrow_mut()
                .get_mut(SOURCE)
                .unwrap()
                .as_table_mut()
                .unwrap()
                .entry(CRATES_IO)
                .or_insert(Table(Map::new()));
        }
    }

    /// 如果切换为默认镜像时，则删除 `replace_with` 属性。否则，
    /// 则为 `[source.creates-io]` 添加 `replace-with` 属性，
    /// 该属性用于指示要使用的外部镜像的名称。
    fn replace_with(&self, registry_name: &str) {
        self.fill_crates_io();

        // 去除属性
        if registry_name.eq(RUST_LANG)
            && self
                .data
                .borrow()
                .get(SOURCE)
                .unwrap()
                .get(CRATES_IO)
                .is_some()
        {
            self.data.borrow_mut().get_mut(SOURCE).unwrap()[CRATES_IO]
                .as_table_mut()
                .unwrap()
                .remove(REPLACE_WITH);

            return;
        }

        // 追加属性
        self.data.borrow_mut().get_mut(SOURCE).unwrap()[CRATES_IO]
            .as_table_mut()
            .unwrap()
            .insert(REPLACE_WITH.to_string(), VString(registry_name.to_string()));
    }

    /// 从 `Cargo` 配置文件中获取正在使用的镜像，其中 `rust-lang` 是 `Cargo` 默认使用的镜像。
    pub fn current(&self) -> (String, Option<String>) {
        let rust_lang = RUST_LANG.to_string();

        // 从配置文件中获取镜像名
        let name =
            match self.data.borrow_mut().get_mut(SOURCE).unwrap()[CRATES_IO].get(REPLACE_WITH) {
                Some(current) => match current {
                    VString(str) => str.to_string(),
                    _ => rust_lang,
                },
                None => rust_lang,
            };

        // 从配置文件中根据镜像名获取镜像地址
        let data = self.data.borrow();
        let source_name = data.get(SOURCE).unwrap().as_table().unwrap().get(&name);
        let addr = match source_name {
            Some(value) => Some(
                value
                    .as_table()
                    .unwrap()
                    .get(REGISTRY)
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string(),
            ),
            None => None,
        };

        (name, addr)
    }

    /// 在 `Cargo` 配置文件中添加新的 `[source.xxx]` 镜像属性，并为其指定 `registry` 属性。
    /// `registry` 属性是强制添加的，`${CARGO_HOME}/.cargo/config` 文件中如果存在则会覆盖，
    fn append_registry(&mut self, registry_name: &str, addr: String) {
        // 如果没有 `[source.xxx]` 属性
        if self
            .data
            .borrow_mut()
            .get_mut(SOURCE)
            .unwrap()
            .as_table()
            .unwrap()
            .get(registry_name)
            .is_none()
        {
            let mut m = Map::new();
            m.insert(REGISTRY.to_string(), VString(addr));

            self.data
                .borrow_mut()
                .get_mut(SOURCE)
                .unwrap()
                .as_table_mut()
                .unwrap()
                .insert(registry_name.to_string(), Table(m));

            return;
        }

        self.data
            .borrow_mut()
            .get_mut(SOURCE)
            .unwrap()
            .as_table_mut()
            .unwrap()
            .get_mut(registry_name)
            .unwrap()
            .as_table_mut()
            .unwrap()
            // NOTE: 这里不能用 `or_insert()`
            .insert(REGISTRY.to_string(), VString(addr));
    }

    /// 切换 `Cargo` 配置文件中正在使用的镜像
    pub fn use_registry(
        &mut self,
        registry_name: &str,
        registry_addr: Option<&String>,
    ) -> Result<(), String> {
        if registry_addr.is_none() {
            return Err(registry_name.to_string());
        }

        self.replace_with(registry_name);

        if registry_name.eq(RUST_LANG) {
            return Ok(());
        }

        self.append_registry(registry_name, registry_addr.unwrap().to_string());

        Ok(())
    }
}
