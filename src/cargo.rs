//! # 处理 `Cargo` 配置
//!
//! 该模块会解析 `Cargo` 配置，并将其反序列化为对象的形式，修改完成后再序列化为相应的文件。
//! `CargoConfig` 是一个操作 `Cargo` 配置文件的对象，有了它一切都好办了。

use std::process;

use toml_edit::{table, value};

use crate::{
    constants::{
        BIAO, CARGO_CONFIG_PATH, CRATES_IO, REGISTRY, REPLACE_WITH, RUST_LANG, SOURCE, ZI_FU_CHUAN,
    },
    description::RegistryDescription,
    toml::Toml,
    util::{cargo_config_path, field_eprint, get_cargo_config},
};

/// `Cargo` 配置对象
pub struct CargoConfig {
    /// 配置对象中的数据，它是一个经过反序列化的对象
    data: Toml,
}

impl CargoConfig {
    /// 创建配置对象
    pub fn new() -> Self {
        let toml = get_cargo_config();

        // 如果文件是空的
        if toml.trim().is_empty() {
            return CargoConfig {
                data: Toml::parse("[source]").unwrap(),
            };
        }

        match Toml::parse(&toml) {
            Ok(mut config) => {
                let data = config.table_mut();
                let source = &data[SOURCE];

                // 如果没有则创建表，否则判断是不是表
                if source.is_none() {
                    data[SOURCE] = table();
                } else if !source.is_table() {
                    field_eprint(SOURCE, BIAO);
                    process::exit(-1);
                }

                CargoConfig { data: config }
            }
            Err(_) => {
                eprint!("{} 文件解析失败，请修改后重试", CARGO_CONFIG_PATH);
                process::exit(-1);
            }
        }
    }

    /// 将 `Cargo` 配置写入到文件中
    pub fn make(&self) {
        self.data.write(cargo_config_path())
    }

    /// 如果 `Cargo` 配置文件中不包含 `[source.crates-io]` 属性，则为 `Cargo` 配置自动填充。
    fn fill_crates_io(&mut self) {
        let data = self.data.table_mut();
        let crates_io = &data[SOURCE][CRATES_IO];

        if crates_io.is_none() {
            data[SOURCE][CRATES_IO] = table();
        } else if !crates_io.is_table() {
            field_eprint(CRATES_IO, BIAO);
            process::exit(-1);
        }
    }

    /// 如果切换为默认镜像时，则删除 `replace_with` 属性。否则，
    /// 则为 `[source.creates-io]` 添加 `replace-with` 属性，
    /// 该属性用于指示要使用的外部镜像的名称。
    fn replace_with(&mut self, registry_name: &str) {
        self.fill_crates_io();

        let data = self.data.table_mut();
        let crates_io = &mut data[SOURCE][CRATES_IO];

        // 去除属性
        if registry_name.eq(RUST_LANG) && !crates_io.is_none() {
            crates_io.as_table_mut().unwrap().remove(REPLACE_WITH);
            return;
        }

        // 追加属性
        crates_io[REPLACE_WITH] = value(registry_name);
    }

    /// 从 `Cargo` 配置文件中获取正在使用的镜像，其中 `rust-lang` 是 `Cargo` 默认使用的镜像。
    pub fn current(&mut self) -> (String, Option<String>) {
        let data = self.data.table_mut();
        let replace_with = &data[SOURCE][CRATES_IO][REPLACE_WITH];

        // 从配置文件中获取镜像名
        let name = if !replace_with.is_none() {
            match replace_with.as_str() {
                Some(name) => name,
                None => {
                    field_eprint(REPLACE_WITH, ZI_FU_CHUAN);
                    process::exit(-1);
                }
            }
        } else {
            RUST_LANG
        };

        // 从配置文件中根据镜像名获取镜像地址
        let addr = if let Some(v) = &data[SOURCE][name][REGISTRY].as_str() {
            Some(v.to_string())
        } else {
            None
        };

        (name.to_string(), addr)
    }

    /// 在 `Cargo` 配置文件中添加新的 `[source.xxx]` 镜像属性，并为其指定 `registry` 属性。
    /// `registry` 属性是强制添加的，`${CARGO_HOME}/.cargo/config` 文件中如果存在则会覆盖，
    fn append_registry(&mut self, registry_name: &str, addr: String) {
        let source = &mut self.data.table_mut()[SOURCE];
        let registry = &source[registry_name];

        // 如果没有 `[source.xxx]` 属性
        if registry.is_none() {
            source[registry_name] = table();
        } else if !registry.is_table() {
            field_eprint(registry_name, BIAO);
            process::exit(-1);
        }

        source[registry_name][REGISTRY] = value(addr);
    }

    /// 根据镜像名删除 `config` 中的旧的镜像属性
    fn remove_old_registry(&mut self, registry_name: &str) {
        if registry_name.eq(RUST_LANG) {
            return;
        }

        let source = &mut self.data.table_mut()[SOURCE];

        // 如果没有 `[source.xxx]` 属性
        if source[registry_name].is_none() {
            return;
        }

        source
            .as_table_mut()
            .unwrap()
            .remove(registry_name)
            .unwrap();
    }

    /// 切换 `Cargo` 配置文件中正在使用的镜像
    pub fn use_registry(
        &mut self,
        registry_name: &str,
        registry_description: Option<&RegistryDescription>,
    ) -> Result<(), String> {
        if registry_description.is_none() {
            return Err(registry_name.to_string());
        }

        // 获取老的镜像名
        let (old_name, _) = self.current();

        // 替换镜像源
        self.replace_with(registry_name);

        // 删除老的镜像属性
        self.remove_old_registry(&old_name);

        if registry_name.eq(RUST_LANG) {
            return Ok(());
        }

        // 追加新的镜像属性
        self.append_registry(
            registry_name,
            registry_description.unwrap().registry.to_string(),
        );

        Ok(())
    }
}
