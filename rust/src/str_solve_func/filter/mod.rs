pub mod char_filter;
pub mod func;
//pub mod str_filter;
pub use char_filter::*;
pub use func::*;
use serde::{Deserialize, Serialize};
use crate::str_solve_func::StrSolveFuncExec;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StrSolveFuncFilter {
	#[serde(default="default_filter_mode")]
	mode: FilterMode,
	script: FilterType,
}

impl StrSolveFuncExec for StrSolveFuncFilter {
	fn exec(&self, input: &str) -> Vec<String>
	{
		if matches!(self.script,FilterType::Zh) {
			if  !contains_category(input, is_hangeul) && !contains_category(input, is_kana) && contains_category(input, is_hanzi) {
				vec![input.to_string()]
			} else {
				vec![]
			}} else {
			if self.mode.exec(input, self.script.get_operation()) {
				return vec![input.to_string()]
			}
			else {
				return vec![]
			}
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum FilterMode {
	No,
	Include,
	Only,
	DoNotCare,
}

impl FilterMode {
	pub  fn exec<T: Fn(char) -> bool>(&self,input: &str ,operation :T) -> bool {
		match self {
			FilterMode::Include => contains_category(input, operation),
			FilterMode::Only => is_entirely_category(input, operation),
			FilterMode::No => !contains_category(input, operation),
			FilterMode::DoNotCare => true,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum FilterType {
	Ascii,
	Unicode,

	Latin,
	Greek,
	Cyrillic,

	//CJK
	Cjk,

	Hanzi,
	Kana,
	Hangeul,

	Zh,
	Jp,
	Ko,
}

impl FilterType {
	pub fn get_operation (&self) -> fn(char) -> bool
	{
		match self {
			FilterType::Ascii => is_ascii,
			FilterType::Unicode => is_unicode,
			FilterType:: Latin => is_latin,
			FilterType::Greek => is_greek,
			FilterType::Cyrillic => is_cyrillic,
			FilterType::Cjk => is_cjk,
			FilterType::Hanzi => is_hanzi,
			FilterType::Kana => is_kana,
			FilterType::Hangeul => is_hangeul,
			FilterType::Zh => is_zh,
			FilterType::Jp => is_jp,
			FilterType::Ko => is_ko,
		}
	}
}

fn default_filter_mode() ->  FilterMode {
	FilterMode::Include
}
