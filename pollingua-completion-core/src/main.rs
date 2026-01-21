mod str_solve_tree;
pub mod str_solve_func;
use str_solve_tree::ExecMode;
use str_solve_tree::StrSolveTree;

//use std::env::args;
//use rayon::prelude::*;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

mod env;
use env::*;

fn check_and_init() -> String
{
	if !check_tmpfile_jp_dict() {
		mk_tmpfile_jp_dict()
	}
	let json_ret = read_tmpfile_func_json();
	let json = match  json_ret {
		Ok(x) => x,
		Err(_) => mk_tmpfile_func_json(),
	};
	json
}

fn get_filename_list(path: &Path) -> Vec<String> {
	if path.is_dir() {
		fs::read_dir(path)
		.expect("Cannot read dir")
		// 1. read_dir 返回的是 Result<DirEntry>，需要处理错误
		.filter_map(|entry| {
			entry.ok().and_then(|e| {
				// 2. 获取文件名并尝试转换为 UTF-8 字符串
				e.file_name().into_string().ok()
			})
		})
		.collect()
	} else {
		vec![]
	}
}

fn main() {
	let mut args = std::env::args();
	let _arg0 = args.nth(0);
	let arg1 = args.nth(0);
	let arg2 = args.nth(0);
	let str_list = get_filename_list(&PathBuf::from(arg2.expect("not arg found.")));
	let json = check_and_init();
	let tree = StrSolveTree::from_json_str(&json).unwrap();
	//let test_str_list: Vec<String> = vec!["複雜test猪🐖测试","대한test","太郎は次郎が持っている本を花子に渡したtest","TEST"].into_iter().map(|x| x.to_string()).collect();
	let ret = tree.get_completion(&str_list, ExecMode::Parallel, &arg1.expect("not arg found."));
	println!("{}",ret.join("\n"));
}
