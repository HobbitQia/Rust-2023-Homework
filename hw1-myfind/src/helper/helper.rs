/*  helper.rs
    helper 模块用来查找路径中匹配的文件。
*/

use std::error::Error;
use std::fs;
use std::path::Path;
use regex::Regex;
use std::process;

/*  find
    该函数用来查找路径中匹配的文件。
*/
pub fn find<P: AsRef<Path>>(    
    root: P,    
    regex: &Regex,
    verbose: bool,
) -> Result<Vec<String>, Box<dyn Error>> {
    let mut ans = Vec::new();
    let path = root.as_ref();
    if !path.exists() {   
        eprintln!("目标路径不存在。");
        process::exit(1);
    }
    walk_tree(root.as_ref(), regex, &mut ans, verbose)?;
    Ok(ans)
}

/*  walk_tree
    该函数用来遍历目录树，查找匹配的文件。
*/
pub fn walk_tree(
    dir: &Path,
    regex: &Regex,
    ans: &mut Vec<String>,
    verbose: bool,
) -> Result<(), Box<dyn Error>> {
    if dir.is_dir() {
        // read_dir 方法返回一个迭代器，迭代器的元素是 Result<DirEntry, Error> 类型
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if verbose {
                println!("{}", path.to_string_lossy().to_string());
            }
            if path.is_dir() {
                walk_tree(&path, regex, ans, verbose)?;
            }
            else if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                if regex.is_match(filename) {
                    ans.push(path.to_string_lossy().to_string());
                }
            }
        }
    }
    // 如果 dir 是一个文件，那么直接判断是否匹配
    else if let Some(filename) = dir.file_name().and_then(|s| s.to_str()) {
        if regex.is_match(filename) {
            ans.push(dir.to_string_lossy().to_string());
        }
    }
    Ok(())
}