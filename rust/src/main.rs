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
use std::borrow::Cow;
use itertools::Itertools;

mod get_env;
use get_env::*;
//use get_env::build_json::*;

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

fn shift_input<'a>(input: &'a str, path: &'a  str) -> (Cow<'a, str>, Cow<'a, Path>, Option<Cow<'a, Path>>)
{
	let tmp_path_input = Path::new(input);
	let dirname_res = tmp_path_input.parent();
	match dirname_res {
		Some(dirname) => {
			let ( newpath, prefix ): ( Cow< Path> , Cow<Path> ) = if tmp_path_input.is_absolute() {
				(dirname.into(), dirname.into())
			}
			else  {
				(PathBuf::from(path).join(dirname).into(), dirname.into())
			};
			if newpath.exists() {
				let basename = tmp_path_input.file_name().unwrap_or_else(|| tmp_path_input.as_os_str());
				return (basename.to_string_lossy(), newpath, Some(prefix));
			}
			else {
				return (input.into(), Path::new(path).into(), None)
			}
		}
		None => return (input.into(), Path::new(path).into(), None),
	}
}

fn main() {
	let json = check_and_init();
	let mut args = std::env::args();
	let _arg0 = args.nth(0);
	let arg1 = args.nth(0).unwrap_or_else(|| std::process::exit(1));
	let arg2 = args.nth(0).unwrap_or_else(|| std::process::exit(1));
	let tree = StrSolveTree::from_json_str(&json).unwrap();
	let input_shifted = shift_input(&arg1,&arg2);
	let source = input_shifted.0;
	let path = input_shifted.1;
	let str_list = get_filename_list(&path);
	let ret = tree.get_completion(&str_list, ExecMode::Single, &source);
	let out = match input_shifted.2 {
		Some(p) => {
			let prefix = p.into_owned();
			ret.iter().map(|suffix| prefix.join(suffix).to_string_lossy().into_owned()).join("\n")
		},
		None => ret.join("\n"),
	};
	println!("{}", out);
}
