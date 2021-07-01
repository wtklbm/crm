# crm (Cargo registry manager)

[`crm`](https://github.com/wtklbm/crm) 是一个在终端运行的镜像管理程序，能够对 `Cargo` 镜像源进行简单的添加、修改、删除操作，并能帮助您快速的切换不同的 `Cargo` 镜像源。`crm` 内置了 5 种国内镜像源，它们分别是: `rustcc`, `sjtu`, `tuna`, `ustc`, `rsproxy`。



## 安装

```bash
# 在终端执行

$ cargo install crm
```



## 使用

`crm` 的原则是使用最小依赖，并尽可能的简化终端操作。您只需要在终端键入 `crm` 即可获得命令帮助信息。

```bash
# 在终端执行

$ crm

  crm best                    评估网络延迟并自动切换到最优的镜像
  crm current                 获取当前所使用的镜像
  crm default                 恢复为默认的镜像
  crm list                    从镜像配置文件中获取镜像列表
  crm remove <name>           在镜像配置文件中删除镜像
  crm save <name> <addr> <dl> 在镜像配置文件中添加/更新镜像
  crm test [name]             下载测试包以评估网络延迟
  crm use <name>              切换为要使用的镜像
```



## 注意事项

1. `v0.1.0` 版本以下的 `.crmrc` 配置文件和最新版本的配置文件并不能相互兼容，如果您正在使用小于 `v0.1.0` 的版本，当您更新到最新版本时，请手动删除 `~/.crmrc` 文件
2. `crm` 会修改 `~/.cargo/config` 文件来进行镜像源的切换，如果您使用的是小于 `v0.1.3` 的版本，那么当您使用 `crm` 切换镜像时，`~/.cargo/config` 文件中的文档注释会被删除并且永远无法恢复，如果您在 `~/.cargo/config` 文件中保存了笔记或者文档，请尽快更新到最新版，在最新版中，对此进行了优化，不再自动删除文档注释 (除修改的字段外)


## LICENSE

MIT OR Apache-2.0

