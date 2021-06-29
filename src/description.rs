//! 镜像描述模块
//!
//! 该模块定义了 `RegistryDescription` 结构体

/// 镜像描述
#[derive(Debug)]
pub struct RegistryDescription {
    /// 镜像地址
    pub registry: String,

    /// 镜像 `dl`
    pub dl: String,
}

impl RegistryDescription {
    /// 创建一个镜像描述对象
    pub fn new(registry: String, dl: String) -> Self {
        RegistryDescription { registry, dl }
    }
}
