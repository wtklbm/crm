//! # 解析并处理程序运行时参数
//!
//! 当程序运行时，会使用一些参数来完成一系列的操作，当程序接收到命令行参数时，
//! 由该模块完成对参数的解析和处理。
//!
//! 目前可接收的运行时参数主要包括:
//!   - `add <registry_name> <registry_addr>`: 在镜像配置文件中添加镜像
//!   - `current`: 获取当前所使用的镜像
//!   - `default`: 恢复为默认的镜像
//!   - `list`: 从镜像配置文件中获取镜像列表
//!   - `remove <registry_name>`: 在镜像配置文件中删除镜像
//!   - `update <registry_name> <registry_addr>`: 在镜像配置文件中更新镜像
//!   - `use <registry_name>`: 切换为要使用的镜像
//!
//! 其中，`add`、`update`、`remove` 命令只修改 `${HOME}/.crmrc` 配置文件，
//! 而不对 `${CARGO_HOME}/.cargo/config` 文件做任何的操作。
//! 如果需要操作 `.crmrc` 镜像配置时仍要修改 `config` 配置文件，
//! 请在操作完镜像配置文件后手动执行相应的操作镜像的命令。

use std::env::args_os;

use crate::{registry::Registry, util::not_command};

type Args = (String, Vec<String>);

/// 解析程序运行时所传递的命令行参数
///
/// 当参数解析完之后会返回一个元组，元组的第一项为要执行的命令，
/// 元组的第二项为执行该命令所要用到的参数。
pub fn parse_args() -> Args {
    let mut args_os = args_os().map(|os_string| os_string.into_string().unwrap());

    args_os.next();

    let command = args_os.next();

    if let None = command {
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
        "list" => println!("{}", r.list()),

        // 恢复默认镜像
        "default" => r.default(),

        // 评估网络延迟并使用最优的镜像
        "best" => r.best(),

        // 切换镜像
        "use" => r.select(args.get(0)),

        // 删除镜像
        "remove" => r.remove(args.get(0)),

        // 对镜像源网络延迟进行评估
        "test" => r.test(args.get(0)),

        // 获取当前镜像
        "current" => {
            let (name, addr) = r.current();

            match addr {
                Some(addr) => println!("{}: {}", name, addr),
                None => println!("{}", name),
            };
        }

        command => {
            let name = args.get(0);
            let addr = args.get(1);
            let dl = args.get(2);

            match command {
                // 添加/更新镜像
                "save" => r.save(name, addr, dl),

                _ => not_command(command),
            }
        }
    }
}
