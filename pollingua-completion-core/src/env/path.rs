use std::path::{Path, PathBuf};
use std::env;

pub(super) fn get_tmpdir_path() -> PathBuf {
	env::var("POLINGUA_COMPLETION_RUNTIME_TMPFILE_PATH")
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
