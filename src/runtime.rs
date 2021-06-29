//! # 运行时
//!
//! `runtime` 是一个处理程序运行时配置的模块，通过它，我们可以通过执行的命令来更改 `.crmrc` 文件。
//! 而 `.crmrc` 文件里面存储的是关于 `Cargo` 配置的相关信息。

#![allow(unused)]

use std::{
    collections::HashMap,
    fs::{self, read_to_string, write},
    path::PathBuf,
    process,
};

use toml::{
    self,
    map::Map,
    Value::{self, String as VString, Table},
};

use crate::{
    constants::{CRMRC, CRMRC_FILE, DL, PUBLIC_RC, REGISTRY, SOURCE},
    description::RegistryDescription,
    util::{append_end_spaces, home_dir},
};

/// 运行时配置
#[derive(Debug)]
pub struct RuntimeConfig {
    /// 运行时配置的存放路径
    path: PathBuf,

    /// 用户自定义镜像的映射表
    extend: HashMap<String, RegistryDescription>,

    /// 默认镜像的映射表
    default: HashMap<String, RegistryDescription>,
}

impl RuntimeConfig {
    /// 创建运行时配置对象
    pub fn new() -> Self {
        // 获取运行时配置的保存路径
        let rc_path = home_dir().join(CRMRC);

        // 获取用户自定义镜像配置
        let data = match read_to_string(&rc_path) {
            Ok(content) => content,
            Err(_) => String::new(),
        };

        RuntimeConfig {
            path: rc_path,
            extend: RuntimeConfig::deserialize_map(data),
            default: RuntimeConfig::deserialize_map(CRMRC_FILE.to_string()),
        }
    }

    /// 获取所有的镜像名 `Vec`
    pub fn registry_names(&self) -> Vec<String> {
        self.default
            .iter()
            .chain(self.extend.iter())
            .map(|(k, _)| k.to_string())
            .collect()
    }

    /// 将运行时配置中的镜像列表转换为字符串
    pub fn to_string(&self, mut sep: Option<&str>) -> String {
        let sep = if let None = sep { "" } else { sep.unwrap() };

        self.default
            .iter()
            .chain(self.extend.iter())
            .fold(String::new(), |mut memo, (k, v)| {
                let s = append_end_spaces(k, None);
                memo.push_str(&format! {"{}{}{}\n", s, sep, v.registry });
                memo
            })
            .trim_end()
            .to_string()
    }

    /// 将运行时配置中的镜像列表名转换为字符串
    pub fn to_key_string(&self) -> String {
        let f = |key| format!("  - {}", key);
        let v1 = self.default.keys().map(f);
        let v2 = self.extend.keys().map(f);

        v1.chain(v2).collect::<Vec<String>>().join("\n")
    }

    /// 将运行时配置写入到文件中
    pub fn write(&self) {
        let data = RuntimeConfig::serialize_map(&self.extend);
        fs::write(&self.path, data).unwrap();
    }

    /// 获取运行时配置中的某一个属性
    pub fn get(&self, registry_name: &str) -> Option<&RegistryDescription> {
        match self.get_extend(registry_name) {
            None => self.get_default(registry_name),
            v => v,
        }
    }

    /// 获取用户自定义运行时配置
    pub fn get_extend(&self, registry_name: &str) -> Option<&RegistryDescription> {
        self.extend.get(registry_name)
    }

    /// 获取默认的运行时配置
    pub fn get_default(&self, registry_name: &str) -> Option<&RegistryDescription> {
        self.default.get(registry_name)
    }

    /// 添加/更新运行时配置中的属性
    pub fn save(&mut self, registry_name: &str, registry_addr: &str, registry_dl: &str) {
        self.extend.insert(
            registry_name.to_string(),
            RegistryDescription {
                registry: registry_addr.to_string(),
                dl: registry_dl.to_string(),
            },
        );
    }

    /// 删除运行时配置中的属性
    pub fn remove(&mut self, registry_name: &str) {
        self.extend.remove(registry_name);
    }

    /// 反序列化 `HashMap`
    fn deserialize_map(data: String) -> HashMap<String, RegistryDescription> {
        let mut config = toml::from_str::<Value>(&data);

        if let Err(_) = config {
            eprint!("由于版本更新，旧版的配置文件已不再兼容，请删除 ~/.crmrc 文件后重试");
            process::exit(-1);
        }

        let mut config = config.unwrap();

        // 如果配置文件中的 `source` 属性不是 `Table`，则创建一个空的 `Table` 赋值给 `source`。
        if config.get(SOURCE).is_none() || !config.get(SOURCE).unwrap().is_table() {
            config
                .as_table_mut()
                .unwrap()
                .entry(SOURCE)
                .or_insert(Table(Map::new()));
        }

        let mut map = HashMap::new();

        // NOTE: 除了 `source` 属性之外，其他属性将被忽略
        // 即使在 `.crmrc` 中有其他属性，在写入到配置文件时也会被自动删除
        let source_table = config.get_mut(SOURCE).unwrap().as_table().unwrap();

        source_table.iter().for_each(|(key, value)| {
            let table = value.as_table().unwrap();
            let registry = table.get(REGISTRY).unwrap().as_str().unwrap().to_string();
            let dl = table.get(DL).unwrap().as_str().unwrap().to_string();

            map.insert(key.to_owned(), RegistryDescription::new(registry, dl));
        });

        map
    }

    /// 序列化 `HashMap`
    fn serialize_map(data: &HashMap<String, RegistryDescription>) -> String {
        let mut sorted_keys = data.keys().collect::<Vec<&String>>();
        sorted_keys.sort_by(|a, b| a.cmp(b));

        let data = sorted_keys.iter().fold(String::new(), |mut memo, k| {
            let v = data.get(*k).unwrap();
            memo.push_str(
                &format! {"[source.{}]\nregistry = \"{}\"\ndl = \"{}\"\n\n", k, v.registry, v.dl },
            );
            memo
        });

        data.trim().to_string()
    }
}
