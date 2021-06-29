# crm ([crates.io](https://crates.io) registry manager)

[`crm`](https://github.com/wtklbm/crm) 是一个在终端运行的镜像管理程序，能够对 `crates.io` 镜像源进行简单的添加、修改、删除操作，并能帮助您快速的切换不同的 `crates.io` 镜像源。`crm` 内置了 5 种国内镜像源，它们分别是: `rustcc`, `sjtu`, `tuna`, `ustc`, `rsproxy`。



## 安装

```bash
# 在终端执行

$ cargo install crm
```



## 使用

`crm` 的原则是使用最小依赖，并尽可能的简化终端操作。您需要在终端键入 `crm` 即可获得命令描述信息。

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



## LICENSE

MIT OR Apache-2.0

