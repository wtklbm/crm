//! # 解析并处理程序运行时参数
//!
//! 当程序运行时，会使用一些参数来完成一系列的操作，当程序接收到命令行参数时，
//! 由该模块完成对参数的解析和处理。
//!
//! 目前可接收的运行时参数主要包括:
//!   - `crm best`:                    评估网络延迟并自动切换到最优的镜像
//!   - `crm best git`:                仅评估 git 镜像源
//!   - `crm best sparse`:             仅评估支持 sparse 协议的镜像源
//!   - `crm best git-download`:       仅评估能够快速下载软件包的 git 镜像源 (推荐使用)
//!   - `crm best sparse-download`:    仅评估能够快速下载软件包且支持 sparse 协议的镜像源 (推荐使用)
//!   - `crm current`:                 获取当前所使用的镜像
//!   - `crm default`:                 恢复为官方默认镜像
//!   - `crm install [args]`:          使用官方镜像执行 `cargo install`
//!   - `crm list`:                    从镜像配置文件中获取镜像列表
//!   - `crm publish [args]`:          使用官方镜像执行 `cargo publish`
//!   - `crm remove <name>`:           在镜像配置文件中删除镜像
//!   - `crm save <name> <addr> <dl>`: 在镜像配置文件中添加/更新镜像
//!   - `crm test [name]`:             下载测试包以评估网络延迟
//!   - `crm update [args]`:           使用官方镜像执行 `cargo update`
//!   - `crm use <name>`:              切换为要使用的镜像
//!   - `crm version`:                 查看当前版本
//!   - `crm check-update`:            检测版本更新
//!
//! 其中，`save`、`remove` 命令只修改 `${HOME}/.crmrc` 配置文件，
//! 而不对 `${CARGO_HOME}/.cargo/config` 文件做任何的操作。
//! 如果需要操作 `.crmrc` 镜像配置时仍要修改 `config` 配置文件，
//! 请在操作完镜像配置文件后手动执行相应的操作镜像的命令。

use std::env::args_os;

use crate::{
    constants::APP_VERSION,
    registry::Registry,
    utils::{get_newest_version, not_command, to_out},
};

type Args = (String, Vec<String>);

/// 解析程序运行时所传递的命令行参数
///
/// 当参数解析完之后会返回一个元组，元组的第一项为要执行的命令，
/// 元组的第二项为执行该命令所要用到的参数。
pub fn parse_args() -> Args {
    let mut args_os = args_os().map(|os_string| os_string.into_string().unwrap());

    args_os.next();

    let command = args_os.next();

    if command.is_none() {
        not_command("");
    }

    (
        command.unwrap(),
        args_os
            .map(|v| v.trim().to_string())
            .collect::<Vec<String>>(),
    )
}

/// 根据运行时参数来处理要执行的命令
///
/// 该函数传递一个运行时参数对象，其中包括命令和执行命令用到的参数。
pub fn handle_command((command, args): Args) {
    let mut r = Registry::new();

    match command.trim().to_lowercase().as_str() {
        // 列出镜像
        "list" => println!("{}", r.list(&r.current().0)),

        // 恢复默认镜像
        "default" => r.default(),

        // 切换镜像
        "use" => r.select(args.first()),

        // 删除镜像
        "remove" => r.remove(args.first()),

        // 使用官方镜像执行 `cargo publish`
        "publish" => r.publish(args.join(" ")),

        // 使用官方镜像执行 `cargo update`
        "update" => r.update(args.join(" ")),

        // 使用官方镜像执行 `cargo install`
        "install" => r.install(args.join(" ")),

        // 对镜像源网络延迟进行评估
        "test" => r.test(&r.current().0, args.first()),

        // 获取当前镜像
        "current" => {
            let (name, addr) = r.current();

            match addr {
                Some(addr) => to_out(format!("{}: {}", name, addr)),
                None => to_out(name),
            };
        }

        // 查看当前的版本
        "version" => {
            println!("  crm v{APP_VERSION}");
        }

        // 检查版本更新
        "check-update" => {
            if let Some(newest) = get_newest_version() {
                if newest != APP_VERSION {
                    return println!("  检测到新版本: {newest}，请切换到官方镜像源以执行更新");
                }
            };

            println!("  暂无更新");
        }

        command => {
            let name = args.first();
            let addr = args.get(1);
            let dl = args.get(2);

            match command {
                // 评估网络延迟并自动切换到最优的镜像
                "best" => r.best(name),

                // 添加/更新镜像
                "save" => r.save(name, addr, dl),

                _ => not_command(command),
            }
        }
    }
}
