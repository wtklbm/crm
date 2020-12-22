//! # 常量
//!
//! 该模块会存储一些程序用到的常量。

/// `${CARGO_HOME}/.cargo/config` 文件中的 `[source]` 属性
pub const SOURCE: &str = "source";

/// `${CARGO_HOME}/.cargo/config` 文件中的 `[source.crates-io]` 属性
pub const CRATES_IO: &str = "crates-io";

/// `${CARGO_HOME}/.cargo/config` 文件中的 `[source.crates-io]` 属性下面的 `replace-with` 属性
pub const REPLACE_WITH: &str = "replace-with";

/// `${CARGO_HOME}/.cargo/config` 文件中的 `[source.xxx]` 属性下面的 `registry` 属性
pub const REGISTRY: &str = "registry";

/// `cargo` 默认的镜像名
pub const RUST_LANG: &str = "rust-lang";

/// `${HOME}` 目录下的 `.crmrc` 文件的文件名
pub const CRMRC: &str = ".crmrc";

/// 程序根目录的 `.crmrc` 文件路径
pub const PUBLIC_RC: &str = "public/.crmrc";

/// 用户查找 `"CARGO_HOME"` 环境变量
pub const CARGO_HOME: &str = "CARGO_HOME";

/// `"CARGO_HOME"` 目录下的 `.cargo` 文件夹
pub const CARGO: &str = ".cargo";

/// `${CARGO_HOME}/.cargo` 目录下的 `config` 文件
pub const CONFIG: &str = "config";

/// `.crmrc` 配置文件的内容
pub const CRMRC_FILE: &str = r#"
# Grammar description:
# registry_name = registry_addr

# 阿里云
aliyun = https://code.aliyun.com/rustcc/crates.io-index
# 官方源
rust-lang = https://github.com/rust-lang/crates.io-index
# `rustcc` 社区 1，宽带为 `5 MB`
rustcc1 = git://crates.rustcc.cn/crates.io-index
# `rustcc` 社区 2，宽带为 `10 MB`
rustcc2 = git://crates.rustcc.com/crates.io-index
# 上海交通大学
sjtu = https://mirrors.sjtug.sjtu.edu.cn/git/crates.io-index
# 清华大学
tuna = https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git
# 中科大
ustc = git://mirrors.ustc.edu.cn/crates.io-index
"#;
