use crate::str_solve_func::zh_mandarin_pinyincrate::*;
use crate::str_solve_func::zh_mandarin_zhuyincrate::*;
use serde_aux::prelude::*;
use serde::{Deserialize, Serialize};
use crate::str_solve_func::StrSolveFuncExec;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StrSolveFuncZhMandarin {
	#[serde(default="default_output")]
	output: CnOutput,
	#[serde(default="default_cn_capitalize")]
	capitalize: CnCapitalize,
	#[serde(default="default_pinyin_format")]
	format: PinyinStructToStr,
	#[serde(deserialize_with = "deserialize_bool_from_anything")]
	#[serde(default)]
	heteronym: bool,
	#[serde(deserialize_with = "deserialize_bool_from_anything")]
	#[serde(default)]
	tone: bool,
	#[serde(deserialize_with = "deserialize_bool_from_anything")]
	#[serde(default="default_filter_bool")]
	filter: bool,
}

impl StrSolveFuncExec for StrSolveFuncZhMandarin {
	fn exec(&self, input: &str) -> Vec<String>
	{
		match self.output {
			CnOutput::Pinyin => {
				let tmp = StrSolveFuncCnPinyin {
					capitalize: self.capitalize,
					format: self.format,
					heteronym: self.heteronym,
					filter: self.filter,
					tone: self.tone,
				};
				tmp.exec(input)
			},
			CnOutput::Zhuyin => {
				let tmp = StrSolveFuncZhuyin {
					heteronym: self.heteronym,
					filter: self.filter,
					tone: self.tone,
				};
				tmp.exec(input)
			}
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StrSolveFuncCnPinyin {
	#[serde(default="default_cn_capitalize")]
	capitalize: CnCapitalize,
	#[serde(default="default_pinyin_format")]
	format: PinyinStructToStr,
	#[serde(deserialize_with = "deserialize_bool_from_anything")]
	#[serde(default)]
	heteronym: bool,
	#[serde(deserialize_with = "deserialize_bool_from_anything")]
	#[serde(default)]
	tone: bool,
	#[serde(deserialize_with = "deserialize_bool_from_anything")]
	#[serde(default="default_filter_bool")]
	filter: bool,
}

impl StrSolveFuncExec for StrSolveFuncCnPinyin {
	fn exec(&self, input: &str) -> Vec<String>
	{
		let tmp = if self.tone == true {
			PinyinStructToStr::Tone
		} else {
			self.format
		};
		if self.filter == true {
			filtered_hanzi_to_pinyin_list(input, self.capitalize, tmp, self.heteronym)
		} else {
			hanzi_to_pinyin_list(input, self.capitalize, tmp, self.heteronym)
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StrSolveFuncZhuyin {
	#[serde(deserialize_with = "deserialize_bool_from_anything")]
	#[serde(default)]
	tone: bool,
	#[serde(deserialize_with = "deserialize_bool_from_anything")]
	#[serde(default)]
	heteronym: bool,
	#[serde(deserialize_with = "deserialize_bool_from_anything")]
	#[serde(default="default_filter_bool")]
	filter: bool,
}

impl StrSolveFuncExec for StrSolveFuncZhuyin {
	fn exec(&self, input: &str) -> Vec<String>
	{
		if self.filter == true {
			filtered_hanzi_to_zhuyin_list(input, !self.tone, self.heteronym)
		} else {
			hanzi_to_zhuyin_list(input, !self.tone, self.heteronym)
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CnOutput {
	Pinyin,
	Zhuyin,
}

fn default_cn_capitalize() ->  CnCapitalize
{
	CnCapitalize::No
}


fn default_filter_bool() ->  bool
{
	true
}


fn default_pinyin_format() ->  PinyinStructToStr
{
	PinyinStructToStr::Full
}

fn default_output() -> CnOutput
{
	CnOutput::Pinyin
}
