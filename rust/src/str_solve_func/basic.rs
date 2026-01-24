use serde::{Deserialize, Serialize};
use crate::str_solve_func::StrSolveFuncExec;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StrSolveFuncAffix {
	#[serde(default)]
    pub prefix: String,
	#[serde(default)]
    pub suffix: String,
}

impl StrSolveFuncExec for StrSolveFuncAffix {
	fn exec(&self, input: &str) -> Vec<String>
	{
		vec![self.prefix.clone() + input + &(self.suffix)]
	}
}

use unidecode::unidecode;
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StrSolveFuncUnidecode {}

impl StrSolveFuncExec for StrSolveFuncUnidecode {
	fn exec(&self, input: &str) -> Vec<String>
	{
		vec![unidecode(input)]
	}
}

use any_ascii::any_ascii;
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StrSolveFuncAnyAscii {}

impl StrSolveFuncExec for StrSolveFuncAnyAscii {
	fn exec(&self, input: &str) -> Vec<String>
	{
		vec![any_ascii(input)]
	}
}

use deunicode::deunicode;
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StrSolveFuncDeunicode {}

impl StrSolveFuncExec for StrSolveFuncDeunicode {
	fn exec(&self, input: &str) -> Vec<String>
	{
		vec![deunicode(input)]
	}
}

use std::collections::HashSet;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StrSolveFuncUnicodeToAscii {
	#[serde(default)]
	pub unidecode: bool,
	#[serde(default)]
	pub anyascii: bool,
	#[serde(default)]
	pub deunicode: bool,
	#[serde(default)]
	pub all: bool,
}

pub(super) fn unicode_to_ascii(input :&str) -> Vec<String>
{
	HashSet::from([unidecode(input), deunicode(input), any_ascii(input)]).into_iter().collect::<Vec<String>>()
}

impl StrSolveFuncExec for StrSolveFuncUnicodeToAscii {
	fn exec(&self, input: &str) -> Vec<String>
	{
		if self.all == true {
			return unicode_to_ascii(input);
		} else {
			let mut tmp_set = HashSet::new();
			if self.unidecode == true {
				tmp_set.insert(unidecode(input));
			}
			if self.deunicode == true {
				tmp_set.insert(deunicode(input));
			}
			if self.anyascii == true {
				tmp_set.insert(any_ascii(input));
			}
			return tmp_set.into_iter().collect::<Vec<String>>();
		};
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub struct StrSolveFuncUnicodeToAsciiAll {}

impl StrSolveFuncExec for StrSolveFuncUnicodeToAsciiAll {
	fn exec(&self, input: &str) -> Vec<String>
	{
		unicode_to_ascii(input)
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StrSolveFuncIdentity {}

impl StrSolveFuncExec for StrSolveFuncIdentity {
	fn exec(&self, input: &str) -> Vec<String>
	{
		vec![input.to_string()]
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StrSolveFuncRoot {}

impl StrSolveFuncExec for StrSolveFuncRoot {
	fn exec(&self, input: &str) -> Vec<String>
	{
		vec![input.to_string()]
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StrSolveFuncTodo {}

impl StrSolveFuncExec for StrSolveFuncTodo {
	fn exec(&self, _input: &str) -> Vec<String>
	{
		todo!();
	}
}
