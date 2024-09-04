//! # 处理 `Cargo` 配置
//!
//! 该模块会解析 `Cargo` 配置，并将其反序列化为对象的形式，修改完成后再序列化为相应的文件。
//! `CargoConfig` 是一个操作 `Cargo` 配置文件的对象，有了它一切都好办了。

use std::process;

use toml_edit::{table, value, Item, Table};

use crate::{
    constants::{
        CRATES_IO, GIT_FETCH_WITH_CLI, NET, PLEASE_TRY, REGISTRIES, REGISTRY, REPLACE_WITH,
        RUST_LANG, SOURCE, STRING, TABLE,
    },
    description::RegistryDescription,
    toml::Toml,
    utils::{cargo_config_path, field_eprint, get_cargo_config, to_out},
};

/// 验证字段是否存在
fn verify_field_exists(data: &mut Table, key: &str) {
    if data.contains_key(key) {
        if !data[key].is_table() {
            field_eprint(key, TABLE);
            process::exit(5);
        }
    } else {
        data[key] = table();
    };
}

/// `Cargo` 配置对象
#[derive(Debug)]
pub struct CargoConfig {
    /// 配置对象中的数据，它是一个经过反序列化的对象
    data: Toml,
}

impl CargoConfig {
    /// 创建配置对象
    pub fn new() -> Self {
        let toml = get_cargo_config();

        match Toml::parse(&toml) {
            Ok(mut config) => {
                let data: &mut Table = config.table_mut();

                // 如果没有则创建表，否则判断是不是表
                verify_field_exists(data, SOURCE);
                verify_field_exists(data, REGISTRIES);
                verify_field_exists(data, NET);

                let net = data[NET].as_table().unwrap();

                if !net.contains_key(GIT_FETCH_WITH_CLI) {
                    data[NET][GIT_FETCH_WITH_CLI] = value(true);
                }

                CargoConfig { data: config }
            }

            Err(_) => {
                to_out(format!(
                    "{} 文件解析失败，{}",
                    cargo_config_path().display(),
                    PLEASE_TRY
                ));
                process::exit(5);
            }
        }
    }

    /// 将 `Cargo` 配置写入到文件中
    pub fn make(&self) {
        self.data.write(cargo_config_path())
    }

    /// 如果 `Cargo` 配置文件中不包含 `[source.crates-io]` 属性，则为 `Cargo` 配置自动填充。
    fn fill_crates_io(&mut self) {
        let data: &mut Table = self.data.table_mut();
        let source: &mut Table = data[SOURCE].as_table_mut().unwrap();

        if source.contains_key(CRATES_IO) {
            if !source[CRATES_IO].is_table() {
                field_eprint(format!("[{SOURCE}.{CRATES_IO}]"), TABLE);
                process::exit(5);
            }
        } else {
            data[SOURCE][CRATES_IO] = table();
        }
    }

    /// 如果切换为默认镜像时，则删除 `replace_with` 属性。否则，
    /// 则为 `[source.creates-io]` 添加 `replace-with` 属性，
    /// 该属性用于指示要使用的外部镜像的名称。
    fn replace_with(&mut self, registry_name: &str) {
        self.fill_crates_io();

        let data: &mut Table = self.data.table_mut();
        let crates_io: &mut Item = &mut data[SOURCE][CRATES_IO];

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
        let source = data[SOURCE].as_table_mut().unwrap();

        // 如果 `source` 不包含 `CRATES_IO` 键，则初始化它
        if !source.contains_key(CRATES_IO) {
            source[CRATES_IO] = table();
        }

        let name = {
            let crates_io = source[CRATES_IO].as_table().unwrap();

            // 从配置文件中获取镜像名
            if crates_io.contains_key(REPLACE_WITH) {
                match crates_io[REPLACE_WITH].as_value().unwrap().as_str() {
                    Some(name) => name,
                    None => {
                        field_eprint(
                            format!("[{SOURCE}.{CRATES_IO}] 下的 {REPLACE_WITH}"),
                            STRING,
                        );
                        process::exit(5);
                    }
                }
            } else {
                RUST_LANG
            }
        };

        if !source.contains_key(name) {
            return (name.to_string(), None);
        }

        let source_name = source[name].as_table().unwrap();

        if !source_name.contains_key(REGISTRY) {
            return (name.to_string(), None);
        }

        let addr = source_name[REGISTRY].as_str().map(|v| v.to_string());

        (name.to_string(), addr)
    }

    /// 追加属性
    fn append_attribute(&mut self, key: &str, registry_name: &str, addr: &str) {
        let config: &mut Table = self.data.table_mut();
        let source: &mut Item = &mut config[key];

        match source.get(registry_name) {
            Some(x) => {
                if !x.is_table() {
                    field_eprint(registry_name, TABLE);
                    process::exit(5);
                }
            }
            None => source[registry_name] = table(),
        };

        let attr = match key {
            SOURCE => REGISTRY,
            REGISTRIES => "index",
            _ => {
                to_out(format!("{key} 不是预期的属性名"));
                process::exit(6);
            }
        };

        // 不管之前存在的值是什么，都要替换成新的值
        source[registry_name][attr] = value(addr.to_string());
    }

    /// 在 `Cargo` 配置文件中添加新的 `[source.xxx]` 镜像属性，并为其指定 `registry` 属性。
    /// `registry` 属性是强制添加的，`${CARGO_HOME}/.cargo/config` 文件中如果存在则会覆盖。
    fn append_registry(&mut self, registry_name: &str, addr: String) {
        self.append_attribute(SOURCE, registry_name, &addr);
    }

    /// 在 `Cargo` 配置文件中添加新的 `[registries.xxx]` 镜像属性，并为其指定 `index` 属性。
    /// `index` 属性是强制添加的，`${CARGO_HOME}/.cargo/config` 文件中如果存在则会覆盖。
    fn append_registries(&mut self, remaining_registries: &[(&str, &str)]) {
        remaining_registries
            .iter()
            .for_each(|(registry_name, registry_addr)| {
                self.append_attribute(REGISTRIES, registry_name, registry_addr);
            });
    }

    /// 删除老的属性
    fn remove_attribute(&mut self, key: &str, registry_name: &str) {
        if registry_name.eq(RUST_LANG) {
            return;
        }

        let source: &mut Item = &mut self.data.table_mut()[key];

        // 如果没有 `[source.xxx]` 属性
        if let None = source.get(registry_name) {
            return;
        }

        source
            .as_table_mut()
            .unwrap()
            .remove(registry_name)
            .unwrap();
    }

    /// 根据镜像名删除 `config` 中的旧的镜像属性
    fn remove_old_registry(&mut self, registry_name: &str) {
        self.remove_attribute(SOURCE, registry_name);
    }

    fn remove_old_registries(&mut self, remaining_registries: &[(&str, &str)]) {
        remaining_registries.iter().for_each(|(registry_name, _)| {
            self.remove_attribute(REGISTRIES, registry_name);
        });
    }

    /// 切换 `Cargo` 配置文件中正在使用的镜像
    pub fn use_registry(
        &mut self,
        registry_name: &str,
        registry_description: Option<&RegistryDescription>,
        remaining_registries: Vec<(&str, &str)>,
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
        self.remove_old_registries(&[(registry_name, "")]);
        self.remove_old_registries(&remaining_registries);
        self.append_registries(&remaining_registries);

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

impl Default for CargoConfig {
    fn default() -> Self {
        Self::new()
    }
}
