//! # 常量
//!
//! 该模块会存储一些程序用到的常量。

/// `${CARGO_HOME}/.cargo/config` 文件中的 `[source]` 属性
pub const SOURCE: &str = "source";

pub const NET: &str = "net";

/// 由于网络原因，在项目添加 `dependencies` 后总显示无法连接 `github.com` 。但是终端中却可以正常使用 `git clone` 命令。
/// 如果有遇到同样情况的，可以在 `$HOME/.cargo/config` 文件中内写入：
///
/// ```toml
/// [net]
/// git-fetch-with-cli = true
/// ```
/// 保存之后就解决了 `cargo` 无法连接 `github.com` 的错误
pub const GIT_FETCH_WITH_CLI: &str = "git-fetch-with-cli";

/// `${CARGO_HOME}/.cargo/config` 文件中的 `[source.crates-io]` 属性
pub const CRATES_IO: &str = "crates-io";

/// `${CARGO_HOME}/.cargo/config` 文件中的 `[source.crates-io]` 属性下面的 `replace-with` 属性
pub const REPLACE_WITH: &str = "replace-with";

/// `${CARGO_HOME}/.cargo/config` 文件中的 `[source.xxx]` 属性下面的 `registry` 属性
pub const REGISTRY: &str = "registry";

/// `${CARGO_HOME}/.cargo/config` 文件中的 `[registries.xxx]` 属性名
pub const REGISTRIES: &str = "registries";

/// `cargo` 默认的镜像名
pub const RUST_LANG: &str = "rust-lang";

/// `${HOME}` 目录下的 `.crmrc` 文件的文件名
pub const CRMRC: &str = ".crmrc";

/// `${HOME}` 目录下的 `.crmrc` 文件的文件路径
pub const CRMRC_PATH: &str = "~/.crmrc";

/// 用户查找 `"${CARGO_HOME}"` 环境变量
pub const CARGO_HOME: &str = "CARGO_HOME";

/// `"CARGO"`
pub const CARGO: &str = "cargo";

/// `${CARGO_HOME}` 目录下的 `.cargo` 文件夹
pub const DOT_CARGO: &str = ".cargo";

/// `${CARGO_HOME}/.cargo` 目录下的 `config` 文件
pub const CONFIG: &str = "config";

/// `dl`
pub const DL: &str = "dl";

/// 应用程序名称
pub const APP_NAME: &str = env!("CARGO_PKG_NAME");

/// 应用程序版本号
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

/// 配置文件所在路径
pub const CARGO_CONFIG_PATH: &str = "$CARGO_HOME/.cargo/config";

/// 表
pub const TABLE: &str = "表";

/// 字符串
pub const STRING: &str = "字符串";

/// 请修改/删除后重试
pub const PLEASE_TRY: &str = "请修改/删除后重试";

/// `UNC` 路径前缀
pub const UNC_PREFIX: &str = r"\\?\";

/// 默认镜像内容
pub const CRMRC_FILE: &str = r#"
# `crm` 配置

# 官方源
[source.rust-lang]
registry = "https://github.com/rust-lang/crates.io-index"
dl = "https://crates.io/api/v1/crates"

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

# 北京外国语大学
[source.bfsu]
registry = "https://mirrors.bfsu.edu.cn/git/crates.io-index.git"
dl = "https://crates.io/api/v1/crates"

# 南京大学
[source.nju]
registry = "https://mirror.nju.edu.cn/git/crates.io-index.git"
dl = "https://crates.io/api/v1/crates"

# 哈尔滨工业大学
[source.hit]
registry = "https://mirrors.hit.edu.cn/crates.io-index.git"
dl = "https://crates.io/api/v1/crates"
"#;
