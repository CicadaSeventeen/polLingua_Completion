use crate::str_solve_func::ko_hengeulcrate::*;
use serde::{Deserialize, Serialize};
use crate::str_solve_func::StrSolveFuncExec;
use crate::str_solve_func::basic::unicode_to_ascii;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum KoFormat {
	Jamo,
	Ascii,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StrSolveFuncHangeul {
	#[serde(default="default_ko_capitalize")]
	capitalize: KoCapitalize,
	#[serde(default="default_format")]
	output: KoFormat,
}

impl StrSolveFuncExec for StrSolveFuncHangeul {
	fn exec(&self, input: &str) -> Vec<String>
	{
		match self.output {
			KoFormat::Jamo => vec![self.capitalize.exec(input)],
			KoFormat::Ascii => unicode_to_ascii(&self.capitalize.exec(input)),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StrSolveFuncJamo {}

impl StrSolveFuncExec for StrSolveFuncJamo {
	fn exec(&self, input: &str) -> Vec<String>
	{
		unicode_to_ascii(input)
	}
}

fn default_ko_capitalize() ->  KoCapitalize
{
	KoCapitalize::No
}

fn default_format() ->  KoFormat
{
	KoFormat::Ascii
}
