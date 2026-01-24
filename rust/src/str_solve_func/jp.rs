use crate::str_solve_func::jp_mecab::*;
use crate::str_solve_func::jp_wanakana::*;
use serde::{Deserialize, Serialize};
use crate::str_solve_func::StrSolveFuncExec;
use crate::str_solve_func::basic::unicode_to_ascii;
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum JpOutput {
	Kana,
	Romaji,
	Ascii,
}

impl JpOutput {
	pub fn from (&self, input: &str) -> Vec<String>
	{
		match self {
			JpOutput::Kana => vec![input.to_string()],
			JpOutput::Romaji => vec![kana_to_romaji(input)],
			JpOutput::Ascii => unicode_to_ascii(input),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum KanaOutput {
	Romaji,
	Ascii,
}

impl KanaOutput {
	pub fn from (&self, input: &str) -> Vec<String>
	{
		match self {
			KanaOutput::Romaji => vec![kana_to_romaji(input)],
			KanaOutput::Ascii => unicode_to_ascii(input),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StrSolveFuncJpAll {
	#[serde(flatten)]
	core: StrSolveFuncJp
}

impl StrSolveFuncExec for StrSolveFuncJpAll {
	fn exec(&self, input: &str) -> Vec<String>
	{
		self.core.exec(input)
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StrSolveFuncJp {
	#[serde(deserialize_with = "deserialize_number_from_string")]
	#[serde(default="default_nbest")]
	nbest: u8,
	#[serde(default="default_jp_output")]
	ouput: JpOutput,
}

impl StrSolveFuncExec for StrSolveFuncJp {
	fn exec(&self, input: &str) -> Vec<String>
	{
		let kana_str_vec = if self.nbest == 1 {
			vec![mecab_fast_to_string(input)]
		} else {
			mecab_nbest(self.nbest, input)
		};
		kana_str_vec.iter().flat_map(|x| self.ouput.from(x)).collect()
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StrSolveFuncJpKana {
	#[serde(default="default_kana_output")]
	output: KanaOutput
}

impl StrSolveFuncExec for StrSolveFuncJpKana {
	fn exec(&self, input: &str) -> Vec<String>
	{
		self.output.from(input)
	}
}

fn default_nbest() ->  u8
{
	1
}

fn default_kana_output() ->  KanaOutput
{
	KanaOutput::Romaji
}

fn default_jp_output() ->  JpOutput
{
	JpOutput::Romaji
}
