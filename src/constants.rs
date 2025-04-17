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
/// @note 可以使用环境变量 `export CARGO_NET_GIT_FETCH_WITH_CLI="true"` 来进行覆盖
/// @reference https://doc.rust-lang.org/cargo/reference/config.html#netgit-fetch-with-cli
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

/// `${CARGO_HOME}/.cargo` 目录下的 `config.toml` 文件 (推荐)
pub const CONFIG_TOML: &str = "config.toml";

/// `dl`
pub const DL: &str = "dl";

/// 应用程序名称
pub const APP_NAME: &str = env!("CARGO_PKG_NAME");

/// 应用程序版本号
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

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

# 上海交通大学 - sparse
[source.sjtu-sparse]
registry = "sparse+https://mirrors.sjtug.sjtu.edu.cn/crates.io-index/"
dl = "https://mirror.sjtu.edu.cn/crates.io/crates/{crate}/{crate}-{version}.crate"

# 中科大
[source.ustc]
registry = "https://mirrors.ustc.edu.cn/crates.io-index"
dl = "https://crates-io.proxy.ustclug.org/api/v1/crates"

# 中科大 - sparse
[source.ustc-sparse]
registry = "sparse+https://mirrors.ustc.edu.cn/crates.io-index/"
dl = "https://crates-io.proxy.ustclug.org/api/v1/crates"

# 字节跳动
[source.rsproxy]
registry = "https://rsproxy.cn/crates.io-index"
dl = "https://rsproxy.cn/api/v1/crates"

# 字节跳动 - sparse
[source.rsproxy-sparse]
registry = "sparse+https://rsproxy.cn/index/"
dl = "https://rsproxy.cn/api/v1/crates"

# 清华大学
[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"
dl = "https://crates.io/api/v1/crates"

# 清华大学 - sparse
[source.tuna-sparse]
registry = "sparse+https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/"
dl = "https://crates.io/api/v1/crates"

# 北京外国语大学
[source.bfsu]
registry = "https://mirrors.bfsu.edu.cn/git/crates.io-index.git"
dl = "https://crates.io/api/v1/crates"

# 北京外国语大学 - sparse
[source.bfsu-sparse]
registry = "sparse+https://mirrors.bfsu.edu.cn/crates.io-index/"
dl = "https://crates.io/api/v1/crates"

# 南京大学
[source.nju]
registry = "https://mirror.nju.edu.cn/git/crates.io-index.git"
dl = "https://crates.io/api/v1/crates"

# 重庆大学 - sparse
[source.cqu-sparse]
registry = "sparse+https://mirrors.cqu.edu.cn/crates.io-index/"
dl = "https://crates.io/api/v1/crates"

# 浙江大学 - sparse
[source.zju-sparse]
registry = "sparse+https://mirrors.zju.edu.cn/crates.io-index/"
dl = "https://crates.io/api/v1/crates"

# CERNET聚合镜像
[source.cernet]
registry = "https://mirrors.cernet.edu.cn/crates.io-index.git"
dl = "https://crates.io/api/v1/crates"

# CERNET聚合镜像 - sparse
[source.cernet-sparse]
registry = "sparse+https://mirrors.cernet.edu.cn/crates.io-index/"
dl = "https://crates.io/api/v1/crates"

# 阿里云 - sparse
[source.aliyun-sparse]
registry = "sparse+https://mirrors.aliyun.com/crates.io-index/"
dl = "https://crates.io/api/v1/crates"
"#;
