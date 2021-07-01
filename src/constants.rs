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

/// `dl`
pub const DL: &str = "dl";

/// 应用程序名称
pub const APP_NAME: &str = "crm";

/// 应用程序版本号
pub const APP_VERSION: &str = "0.1.3";

/// 配置文件所在路径
pub const CARGO_CONFIG_PATH: &str = "$CARGO_HOME/.cargo/config";

/// 表
pub const BIAO: &str = "表";

/// 字符串
pub const ZI_FU_CHUAN: &str = "字符串";

/// 默认镜像内容
pub const CRMRC_FILE: &str = r#"
# `crm` 配置

# 官方源
[source.rust-lang]
registry = "https://github.com/rust-lang/crates.io-index"
dl = "https://crates.io/api/v1/crates"

# `rustcc` 社区
[source.rustcc]
registry = "git://crates.rustcc.cn/crates.io-index"
dl = "https://crates.rustcc.cn/api/v1/crates"

# 上海交通大学
[source.sjtu]
registry = "https://mirrors.sjtug.sjtu.edu.cn/git/crates.io-index"
dl = "https://mirror.sjtu.edu.cn/crates.io/crates/{crate}/{crate}-{version}.crate"

# 清华大学
[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"
dl = "https://crates.io/api/v1/crates"

# 中科大
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"
dl = "https://crates-io.proxy.ustclug.org/api/v1/crates"

# 字节跳动
[source.rsproxy]
registry = "https://rsproxy.cn/crates.io-index"
dl = "https://rsproxy.cn/api/v1/crates"
"#;
