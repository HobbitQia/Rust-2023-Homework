/*  main.rs 
    实现终端与 myfind 的交互，通过给定的命令行输入明确模式，调用对应 myfind 中的函数实现功能。
*/

use myfind::*;
use std::env;
use std::process;
use colored::*;

mod helper;     // helper 模块主要用来查找路径中匹配的文件
mod myfind;

fn main() {
    let args: Vec<String> = env::args().collect();  // 获取命令行参数

    if args.len() < 4 {     
        eprintln!("使用方式：{} [-n1|-n2] [-v|--verbose] <目标目录> <要搜索的正则表达式>", args[0].red());
        process::exit(1);
    }

    let cmd: String = args[1].to_string();
    let ans;
    // let mut ans = Vec::new();
    let verbose: bool = args[2].eq("-v") || args[2].eq("--verbose");    // 表示是否需要输出路径下的所有文件

    match &cmd as &str {
        "-n1" => {       // -n1 参数表示只有一个路径（可以有多个正则表达式）
            let index = if verbose { 4 } else { 3 }; // 表示正则表达式是从第 index 个命令行参数开始
            // let ans = myfind_n1(&args, index, verbose)?;
            ans = match myfind_n1(&args, index, verbose) {
                Ok(matches) => matches,
                Err(error) => {
                    eprintln!("发生错误：{}", error);
                    process::exit(1);
                }
            };
        },
        "-n2" => {     // -n2 参数表示只有一个正则表达式（可以有多个路径）
            let mut index = if verbose { 3 } else { 2 };      
            while args[index].contains("/") || args[index].contains("\\") {
                index += 1;
            };
            ans = match myfind_n2(&args, index, verbose) {
                Ok(matches) => matches,
                Err(error) => {
                    eprintln!("发生错误：{}", error);
                    process::exit(1);
                }
            };
        },
        _ => {
            eprintln!("无效的命令行参数：{}", cmd.red());
            process::exit(1);
        },
    };
    // 输出匹配项
    if ans.is_empty() {
        println!("未找到匹配项。");
    }
    else {
        println!("共找到 {} 个匹配项。", ans.len().to_string().green());
        println!("具体为以下匹配项：");
        for file in ans {
            println!("{}", file.yellow());
        }
    }
}

