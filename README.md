# crm (Cargo registry manager)

[`crm`](https://github.com/wtklbm/crm) 是一个在终端运行的镜像管理程序，能够对 `Cargo` 镜像源进行简单的添加、修改、删除操作，并能帮助您快速的切换不同的 `Cargo` 镜像源。`crm` 内置了 5 种国内 (中国) 镜像源，它们分别是：`rustcc`, `sjtu`, `tuna`, `ustc`, `rsproxy`。


在使用 Rust 语言做开发时，使用 Rust 官方镜像源进行 `cargo build` 的速度非常的慢，可能会因为网络的原因导致依赖下载超时而无法完成编译。为了能够在最少的时间内完成打包操作，一般会使用国内镜像源来代替官方镜像。


通常，大家一般会手动修改 `~/.cargo/config` 文件来完成镜像的切换，手动修改配置文件的工作是繁琐的，它需要手动打开文件所在的目录，还要记住每一个镜像源的地址和配置方式，在不知道哪个国内源的网速最快的时候，我们还需要对镜像的速度进行手动的测速，在使用国内镜像源的过程中，如果当前所使用的国内镜像源也挂了，我们还需要切换到另一个国内镜像源，这就显得非常的棘手。如果您手动配置了国内镜像源，并且还经常的通过 `cargo publish` 发包的话 ，那么在发包之前，还需要将国内镜像源再手动切换为官方镜像。在比如，每一个国内镜像源同步镜像的时间是不一样的，如果您刚发了一个包并且想第一时间应用到您的项目中，但是因为国内镜像源的没有及时的同步镜像，而导致包无法下载，这个时候您还需要切换到官方镜像源来下载最新发布的包。每一次手动切换镜像的操作都是繁琐且耗时的，而 `crm` 就是为了解决上述的问题。



## 安装

### 通过 `cargo` 安装

```bash
# 在终端执行

$ cargo install crm
```



### `Arch` 安装

您可以从 Arch Linux 用户仓库安装它，感谢 [taotieren](https://github.com/taotieren)。

- <https://aur.archlinux.org/packages/crm>
- <https://aur.archlinux.org/packages/crm-git>



## 使用

`crm` 的原则是使用最小依赖，并尽可能的简化终端操作。您只需要在终端键入 `crm` 即可获得命令帮助信息。

```bash
# 在终端执行
#
# NOTE:
#  - [args] 表示 args 是一个或多个可选参数
#  - <name> 表示 name 是一个必填参数
#
# 下面这些命令在执行时会自动切换为官方镜像，避免了手动切换镜像的麻烦：
#  - `crm install` 对应 `cargo install`
#  - `crm publish` 对应 `cargo publish`
#  - `crm update` 对应 `cargo update`

$ crm

  crm best                    评估网络延迟并自动切换到最优的镜像
  crm current                 获取当前所使用的镜像
  crm default                 恢复为官方默认镜像
  crm install [args]          使用官方镜像执行 "cargo install"
  crm list                    从镜像配置文件中获取镜像列表
  crm publish [args]          使用官方镜像执行 "cargo publish"
  crm remove <name>           在镜像配置文件中删除镜像
  crm save <name> <addr> <dl> 在镜像配置文件中添加/更新镜像
  crm test [name]             下载测试包以评估网络延迟
  crm update [args]           使用官方镜像执行 "cargo update"
  crm use <name>              切换为要使用的镜像
```



## 在项目中使用来自不同镜像源的依赖

`crm` 在配置镜像源时，会默认在 `~/.cargo/config` 中多增加一个 `registries` 属性对象，通过增加该属性对象，您就可以在项目中应用来自于不同镜像源的依赖。比如您在使用官方镜像源时，可以通过在项目的 `Cargo.toml` 文件中指定依赖的 `registry` 属性来使用不同的国内镜像源。如果您已经在使用国内镜像源了，那么也可以通过修改 `registry` 属性的方式来切换到其他的国内镜像源。以下是一个示例。




```toml
# Cargo.toml

# 使用官方镜像源时，`registry` 属性可选的值为：`rustcc`, `sjtu`, `tuna`, `ustc`, `rsproxy`
# 如果您已经使用 `crm` 切换到了 `rsproxy` 镜像源，那么 `registry` 属性可选的值则为其他 4 种国内镜像：`rustcc`, `sjtu`, `tuna`, `ustc`
# 以此类推
# 值得注意的是，在使用国内镜像源时，您无法直接通过修改 `registry` 属性的方式使用官方镜像源
# 如果您想使用官方镜像源，那么请在终端执行 `crm default` 来切换到官方镜像

[dependencies]
# 使用 `ustc` 国内镜像来下载 `log` 依赖
log = {version = "0.4.12", registry = "ustc"}

# 使用 `sjtu` 国内镜像来下载 `lazy_static` 依赖
lazy_static = {version = "1.4.0", registry = "sjtu"}
```



> NOTE：如果您刚安装 `crm`，那么请在终端执行一次：`crm default`，然后就可以在项目的 `Cargo.toml` 文件中配置 `registry` 属性了。



## 可选镜像源

下面提供了一些没有被内置到 `crm` 中的可选镜像源。您可以手动执行命令，将他们保存到 `crm` 镜像列表，以备后续使用。

```bash
# 在终端执行

# 北京外国语大学
$ crm save bfsu "https://mirrors.bfsu.edu.cn/git/crates.io-index.git" "https://crates.io/api/v1/crates"

# 南京大学
$ crm save nju "https://mirror.nju.edu.cn/git/crates.io-index.git" "https://crates.io/api/v1/crates"

# 哈尔滨工业大学
$ crm save hit "https://mirrors.hit.edu.cn/crates.io-index.git" "https://crates.io/api/v1/crates"
```



## 注意事项

1. `v0.1.0` 版本以下的 `.crmrc` 配置文件和最新版本的配置文件并不能相互兼容，如果您正在使用小于 `v0.1.0` 的版本，当您更新到最新版本时，请手动删除 `~/.crmrc` 文件
2. `crm` 会修改 `~/.cargo/config` 文件来进行镜像源的切换，如果您使用的是小于 `v0.1.3` 的版本，那么当您使用 `crm` 切换镜像时，`~/.cargo/config` 文件中的文档注释会被删除并且永远无法恢复，如果您在 `~/.cargo/config` 文件中保存了笔记或者文档，请尽快更新到最新版，在最新版中，对此进行了优化，不再自动删除文档注释 (除修改的字段外)
3. `crm` 默认会在 `~/.cargo/config` 文件中增加一个 `env.git-fetch-with-cli` 属性，值为 `true`，在使用 `crm` 时您无法删除该选项，如果您不想使用 `Git` 可执行文件进行 `Git` 操作，请手动修改 `~/.cargo/config` 文件并将 `git-fetch-with-cli` 的值修改为 `false`



## Others

### rust-library-chinese

`rust-library-chinese` 是 Rust 核心库和标准库的源码级中文翻译，可以用作 IDE 工具的中文智能提示，也可以基于翻译好的内容生成 Rust 中文 API 文档。

- [从 Github 访问](https://github.com/wtklbm/rust-library-i18n)
- [从 Gitee 访问](https://gitee.com/wtklbm/rust-library-chinese)




## LICENSE

MIT OR Apache-2.0

