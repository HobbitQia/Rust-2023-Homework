/*  myfind.rs
    myfind 模块用来分别执行不同模式下的查找操作。
*/
use std::error::Error;
use regex::Regex;
use std::process;
use colored::*;

use crate::helper::helper::find;

/*  myfind_n1
    该函数用来实现 -n1 模式下的查找操作。
    -n1 参数表示只有一个路径（可以有多个正则表达式）。
*/
pub fn myfind_n1 (
    args: & Vec<String>,
    index: usize,      // 表示正则表达式是从第 index 个命令行参数开始
    verbose: bool,     // 表示是否需要输出路径下的所有文件
) -> Result<Vec<String>, Box<dyn Error>> {
    let mut ans = Vec::new();
    let mut verbose_tmp = verbose;  // 原因见 line 36
    for i in index..args.len() {
        let pattern = &args[i];
        let regex = match Regex::new(pattern) {
            Ok(re) => re,       // 将要查找的文件名转成 Regex 类，即正则表达式的形式
            Err(err) => {
                eprintln!("无效的正则表达式 '{}'：{}", pattern.red(), err);
                process::exit(1);
            }
        };

        match find(&args[index-1], &regex, verbose_tmp) {
            Ok(tmp_matches) => {
                if i == index { // 第一次查找（最开始 ans 为空，不能求交集）
                    ans.extend(tmp_matches);
                    verbose_tmp = false;    // 我们只应该在第一次遍历的时候输出遍历到的文件，避免重复。
                }
                else {
                    ans = match intersect(&ans, &tmp_matches) {
                        Ok(matches) => matches,
                        Err(err) => {
                            eprintln!("交集操作失败：{}", err);
                            process::exit(1);
                        }
                    };
                }
            }
            Err(error) => {     
                eprintln!("发生错误：{}", error);
                process::exit(1);
            }
        }
    } 
    Ok(ans)
}

/*  myfind_n2
    该函数用来实现 -n2 模式下的查找操作。
    -n2 参数表示只有一个正则表达式（可以有多个路径）。
*/
pub fn myfind_n2 (
    args: & Vec<String>,
    index: usize,       // 表示正则表达式是从第 index 个命令行参数开始
    verbose: bool,      // 表示是否需要输出路径下的所有文件
) -> Result<Vec<String>, Box<dyn Error>> {
    let mut ans = Vec::new();
    let pattern = &args[index];
    let regex = match Regex::new(pattern) {
        Ok(re) => re,       // 将要查找的文件名转成 Regex 类，即正则表达式的形式
        Err(err) => {
            eprintln!("无效的正则表达式 '{}'：{}", pattern.red(), err);
            process::exit(1);
        }
    };
    let start_index = if verbose { 3 } else { 2 };  // 表示路径是从第 start_index 个命令行参数开始
    for i in start_index..index {
        match find(&args[i], &regex, verbose) {
            Ok(tmp_matches) => {
                ans.extend(tmp_matches);
            }
            Err(error) => {     
                eprintln!("发生错误：{}", error);
                process::exit(1);
            }
        }
    }
    Ok(ans)
}

/*  intersect 
    交集操作，求出并返回两个 Vec<String> 的交集。
*/
fn intersect(
    a : &Vec<String> ,
    b : &Vec<String>
) -> Result<Vec<String>, Box<dyn Error>> {
    let mut ans = Vec::new();
    for item in a {
        if b.contains(item) {
            ans.push(item.to_string());
        }
    }
    Ok(ans)
}
