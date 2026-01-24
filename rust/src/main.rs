#[cfg(target_env = "musl")]
use mimalloc::MiMalloc;
#[cfg(target_env = "musl")]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

mod str_solve_tree;
pub mod str_solve_func;
use str_solve_tree::ExecMode;
use str_solve_tree::StrSolveTree;

//use std::env::args;
//use rayon::prelude::*;
#[allow(unused_imports)]
use std::path::Path;
use std::path::PathBuf;

mod get_env;
use get_env::*;

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

fn main() {
	let mut args = std::env::args();
	let _arg0 = args.nth(0);
	let arg1 = args.nth(0);
	let arg2 = args.nth(0);
	let str_list = get_filename_list(&PathBuf::from(arg2.expect("not arg found.")));
	let json = check_and_init();
	let tree = StrSolveTree::from_json_str(&json).unwrap();
	//let test_str_list: Vec<String> = vec!["複雜test猪🐖测试","대한test","太郎は次郎が持っている本を花子に渡したtest","TEST"].into_iter().map(|x| x.to_string()).collect();
	let ret = tree.get_completion(&str_list, ExecMode::Single, &arg1.expect("not arg found."));
	println!("{}",ret.join("\n"));
}
