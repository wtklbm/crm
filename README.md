# `crm -- Cargo registry manager`

[`crm`](https://github.com/wtklbm/crm) 是一个镜像管理程序，能够对镜像源进行简单的添加、修改、删除操作，并能帮助您快速的切换不同的 `cargo` 镜像。`crm` 内置了 5 种国内镜像源，它们分别是: `rustcc`, `sjtu`, `tuna`, `ustc`, `rsproxy`。

# `Install`

```bash
$ cargo install crm
```

# `Example`

```bash
$ crm

  crm add <registry_name> <registry_addr>     在镜像配置文件中添加镜像
  crm current                                 获取当前所使用的镜像
  crm default                                 恢复为默认的镜像
  crm list                                    从镜像配置文件中获取镜像列表
  crm remove <registry_name>                  在镜像配置文件中删除镜像
  crm update <registry_name> <registry_addr>  在镜像配置文件中更新镜像
  crm use <registry_name>                     切换为要使用的镜像
```

# `LICENSE`

MIT OR Apache-2.0
