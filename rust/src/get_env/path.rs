use std::path::{Path, PathBuf};
use std::env;
use serde::Deserialize;
use std::str::FromStr;
use std::fs;

pub(super) fn get_tmpdir_path() -> PathBuf {
	env::var("POLLINGUA_COMPLETION_RUNTIME_TMPFILE_PATH")
		.map(PathBuf::from)
		.ok()
		.or_else(|| {
			env::var("XDG_RUNTIME_DIR")
				.map(|dir| Path::new(&dir).join(".polingua.tmp"))
				.ok()
		})
		.unwrap_or_else(|| PathBuf::from("/tmp/.polingua.tmp"))
}

pub fn get_tmpfile_func_json_path() -> PathBuf {
	get_tmpdir_path().join("func_tree.json")
}

pub  fn get_tmpfile_jp_dict_path() -> PathBuf {
	get_tmpdir_path().join("jp_dict")
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum LsMode {
	All,
	File,
	Dir,
}

// 建议实现 FromStr，这样可以用 .parse()
impl FromStr for LsMode {
	type Err = ();
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"all" => Ok(LsMode::All),
			"file" => Ok(LsMode::File),
			"dir" => Ok(LsMode::Dir),
			_ => Err(()),
		}
	}
}

fn get_filename_list_with_lsmode(path: &Path, mode: LsMode) -> Vec<String> {
	if path.is_dir() {
		fs::read_dir(path)
		.expect("Cannot read dir")
		// 1. read_dir 返回的是 Result<DirEntry>，需要处理错误
		.filter_map(|entry: std::io::Result<fs::DirEntry>| {
			entry.ok().and_then(|e: fs::DirEntry| {
				// 2. 获取文件名并尝试转换为 UTF-8 字符串
				match mode {
					LsMode::All => e.file_name().into_string().ok(),
					LsMode::File => {
						if e.path().is_file() {
							e.file_name().into_string().ok()
						} else {
							None
						}
					}
					LsMode::Dir => {
						if e.path().is_dir() {
							e.file_name().into_string().ok()
						} else {
							None
						}
					}
				}
			})
		})
		.collect()
	} else {
		vec![]
	}
}

pub fn get_filename_list(path: &Path) -> Vec<String> {
	let lsmode: LsMode = env::var("_POLINGUA_COMPLETION_FILETYPE")
	.ok()
	.and_then(|s| s.parse().ok())
	.unwrap_or(LsMode::All);
	get_filename_list_with_lsmode(path, lsmode)
}
