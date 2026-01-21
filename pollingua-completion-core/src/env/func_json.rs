use std::fs;
use super::path::*;
use std::path::Path;
use std::io::Write;
use std::io::{Error, ErrorKind};
use super::build_json::*;

fn mk_tmpfile_func_json_from_path(json: &str, path : &Path) {
	let parent = path.parent().expect("dirname not exist");
	if !parent.exists() {
		fs::create_dir_all(path).unwrap();
	} else if !parent.is_dir() {
		fs::remove_file(parent).unwrap();
		fs::create_dir_all(parent).unwrap();
	}
	if path.exists(){
		if path.is_dir() {
			fs::remove_dir_all(path).unwrap();
		} else {
			fs::remove_file(path).unwrap();
		}
	}
	let mut file_func_json = fs::File::create(path).unwrap();
	file_func_json.write_all(json.as_bytes()).unwrap();
}

fn mk_tmpfile_func_json_from_str(json: &str) {
	mk_tmpfile_func_json_from_path(json, &get_tmpfile_func_json_path());
}

pub fn mk_tmpfile_func_json() -> String
{
	let json = build_json_config_from_env();
	mk_tmpfile_func_json_from_str(&json);
	return json;
}

pub fn read_tmpfile_func_json() -> Result<String, Error>
{
	let path = get_tmpfile_func_json_path();
	let tmp = if path.exists() && path.is_file() {
		fs::read_to_string(path)
	} else {
		Err(Error::new(ErrorKind::Other,"Not json file."))
	};
	tmp
}
