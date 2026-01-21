use std::fs;
use super::path::*;
use std::path::Path;
use std::io::Write;

fn mk_tmpfile_jp_dict_from_path(path : &Path) {
	let bin_mecabrc = include_bytes!("mecabrc");
	let bin_sys_dict = include_bytes!("ipadic-utf8/sys.dic");
	let bin_unk_dict = include_bytes!("ipadic-utf8/unk.dic");
	let bin_char_bin = include_bytes!("ipadic-utf8/char.bin");
	let bin_matrix_bin = include_bytes!("ipadic-utf8/matrix.bin");
	let bin_dicrc = include_bytes!("ipadic-utf8/dicrc");
	if !path.exists() {
		fs::create_dir_all(path).unwrap();
	} else if !path.is_dir() {
		fs::remove_file(path).unwrap();
		fs::create_dir_all(path).unwrap();
	}
	let mut file_mecabrc = fs::File::create(path.join("mecabrc")).unwrap();
	file_mecabrc.write_all(bin_mecabrc).unwrap();
	let mut file_sys_dict = fs::File::create(path.join("sys.dic")).unwrap();
	file_sys_dict.write_all(bin_sys_dict).unwrap();
	let mut file_unk_dict = fs::File::create(path.join("unk.dic")).unwrap();
	file_unk_dict.write_all(bin_unk_dict).unwrap();
	let mut file_char_bin = fs::File::create(path.join("char.bin")).unwrap();
	file_char_bin.write_all(bin_char_bin).unwrap();
	let mut file_matrix_bin = fs::File::create(path.join("matrix.bin")).unwrap();
	file_matrix_bin.write_all(bin_matrix_bin).unwrap();
	let mut file_dicrc = fs::File::create(path.join("dicrc")).unwrap();
	file_dicrc.write_all(bin_dicrc).unwrap();
}

pub fn mk_tmpfile_jp_dict() {
	mk_tmpfile_jp_dict_from_path(&get_tmpfile_jp_dict_path());
}

pub fn check_tmpfile_jp_dict() -> bool
{
	let path = get_tmpfile_jp_dict_path();
	if path.join("sys.dic").is_file() && path.join("unk.dic").is_file() && path.join("char.bin").is_file() && path.join("matrix.bin").is_file() && path.join("dicrc").is_file() {
		return true;
	} else {
		return false;
	}
}
