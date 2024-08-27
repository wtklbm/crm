//! # 运行时
//!
//! `runtime` 是一个处理程序运行时配置的模块，通过它，我们可以通过执行的命令来更改 `.crmrc` 文件。
//! 而 `.crmrc` 文件里面存储的是关于 `Cargo` 配置的相关信息。

use std::{
    collections::{btree_map::Iter, BTreeMap},
    fs::read_to_string,
    iter::Chain,
    path::PathBuf,
    process,
};

use toml_edit::{table, value};

use crate::{
    constants::{
        CRMRC, CRMRC_FILE, CRMRC_PATH, DL, PLEASE_TRY, REGISTRY, RUST_LANG, SOURCE, TABLE,
    },
    description::RegistryDescription,
    toml::Toml,
    utils::{append_end_spaces, home_dir, status_prefix, to_out},
};

/// 运行时配置
#[derive(Debug)]
pub struct RuntimeConfig {
    /// 运行时配置的存放路径
    path: PathBuf,

    /// 用户自定义的配置
    config: Toml,

    /// 用户自定义镜像的映射表
    extend: BTreeMap<String, RegistryDescription>,

    /// 默认镜像的映射表
    default: BTreeMap<String, RegistryDescription>,
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

        let extend = RuntimeConfig::parse(&data);
        let default = RuntimeConfig::parse(CRMRC_FILE);

        RuntimeConfig {
            extend: RuntimeConfig::extract_to_map(&extend),
            default: RuntimeConfig::extract_to_map(&default),
            path: rc_path,
            config: extend,
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
    pub fn to_string(&self, current: &String, sep: Option<&str>) -> String {
        let sep = if sep.is_none() {
            ""
        } else {
            Option::unwrap(sep)
        };

        self.iter()
            .fold(String::new(), |mut memo, (k, v)| {
                let p = status_prefix(k, current);
                let k = append_end_spaces(k, None);
                memo.push_str(format! {"{}{}{}{}\n", p, k, sep, v.registry }.as_str());
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
    pub fn write(&mut self) {
        self.convert_from_map();
        self.config.write(&self.path);
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

    /// 将镜像名称和镜像地址收集到元祖中，并返回一个元祖数组
    pub fn to_tuples(&self, exclude_name: Option<&str>) -> Vec<(&str, &str)> {
        self.iter().fold(vec![], |mut memo, (k, v)| {
            if k.eq(RUST_LANG) || (exclude_name.is_some() && k.eq(exclude_name.unwrap())) {
                return memo;
            }

            memo.push((k, &v.registry));
            memo
        })
    }

    /// 创建迭代器
    fn iter(&self) -> Chain<Iter<String, RegistryDescription>, Iter<String, RegistryDescription>> {
        self.default.iter().chain(self.extend.iter())
    }

    /// 将字符串解析为 `Toml` 对象
    fn parse(data: &str) -> Toml {
        let config = Toml::parse(data);

        if config.is_err() {
            to_out(format!("解析 {} 文件失败，{}", CRMRC_PATH, PLEASE_TRY));
            process::exit(14);
        }

        let mut config = config.unwrap();
        let data = config.table_mut();
        let source = &data[SOURCE];

        // 如果没有则创建表，否则判断是不是表
        if source.is_none() {
            data[SOURCE] = table();
        } else if !source.is_table() {
            to_out(format!(
                "{} 文件中的 {} 字段不是一个{}，{}",
                CRMRC_PATH, SOURCE, TABLE, PLEASE_TRY
            ));
            process::exit(15);
        }

        config
    }

    /// 从配置转换为 `BTreeMap`
    fn extract_to_map(config: &Toml) -> BTreeMap<String, RegistryDescription> {
        let data = config.table();
        let source = data[SOURCE].as_table().unwrap();
        let mut map = BTreeMap::new();

        source
            .iter()
            .for_each(|(key, value)| match value.as_table() {
                Some(v) => {
                    let r = v[REGISTRY].as_str();
                    let d = v[DL].as_str();

                    if r.is_none() || d.is_none() {
                        to_out(format!(
                            "{} 文件中的 [{}.{}] 里没有包含 {} 或 {} 字段, {}",
                            CRMRC_PATH, SOURCE, key, REGISTRY, DL, PLEASE_TRY
                        ));
                        process::exit(16);
                    }

                    let registry = r.unwrap().to_string();
                    let dl = d.unwrap().to_string();

                    map.insert(key.to_string(), RegistryDescription::new(registry, dl));
                }
                None => {
                    to_out(format!(
                        "{} 文件中的 {} 字段不是一个 {}, {}",
                        CRMRC_PATH, key, TABLE, PLEASE_TRY
                    ));
                    process::exit(17);
                }
            });

        map
    }

    /// 从 `BTreeMap` 转换为配置
    fn convert_from_map(&mut self) {
        let config = self.config.table_mut();
        config[SOURCE] = table();
        let source = config[SOURCE].as_table_mut().unwrap();

        self.extend.iter().for_each(|(k, v)| {
            let RegistryDescription { registry, dl } = v;

            source[k] = table();
            source[k][REGISTRY] = value(registry.to_string());
            source[k][DL] = value(dl.to_string());
        });
    }
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self::new()
    }
}
