use pinyin::{ToPinyin, ToPinyinMulti};
use pinyin::Pinyin;
use serde::{Deserialize, Serialize};
use super::filter::*;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum CnCapitalize {
	Initials,
	FirstLetter,
	All,
	No,
}


impl CnCapitalize  {
	pub fn from(&self, input: &str) -> String
	{
		match self {
			CnCapitalize::Initials => capitalize_initials(input),
			CnCapitalize::FirstLetter => capitalize_first(input),
			CnCapitalize::All => input.to_uppercase(),
			CnCapitalize::No => input.to_string(),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum PinyinStructToStr {
	Full,
	FirstLetter,
	Initials,
	Tone,
}

impl PinyinStructToStr {
	pub fn from(&self, pinyin: Pinyin) -> String
	{
		match self {
			PinyinStructToStr::Full => pinyin.plain().to_string(),
			PinyinStructToStr::FirstLetter => pinyin.first_letter().to_string(),
			PinyinStructToStr::Initials => get_initials(pinyin.plain()),
			PinyinStructToStr::Tone => pinyin.with_tone_num_end().to_string(),
		}
	}
}

pub(super) fn hanzi_to_pinyin_list(hanzi :&str, capitalize: CnCapitalize, format: PinyinStructToStr, heteronym :bool) ->  Vec<String>
{
	let func = |x| format.from(x);
	if heteronym == true {
		 pinyin_multi_to_vec(hanzi, capitalize, func)
	} else {
		vec![pinyin_fast_to_string(hanzi, capitalize, func)]
	}
}

pub(super) fn filtered_hanzi_to_pinyin_list(hanzi :&str, capitalize: CnCapitalize, format: PinyinStructToStr, heteronym :bool) ->  Vec<String>
{
	let func = |x: &str|  hanzi_to_pinyin_list(x, capitalize, format, heteronym);
	exec_func_on_matched(hanzi, is_hanzi, func)
}

fn capitalize_first(s: &str) -> String {
	let mut chars = s.chars();
	match chars.next() {
		None => String::new(),
		Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
	}
}

fn capitalize_initials(s: &str) -> String {
	let mut chars = s.chars();
	match chars.next() {
		None => String::new(),
		Some(first) => {
			match &first {
				'z' | 'c' | 's'  => match chars.next() {
					None => first.to_string(),
					Some(second) => match &second {
						'h' => first.to_uppercase().collect::<String>() + "H" + chars.as_str(),
						_  => first.to_uppercase().collect::<String>() + &second.to_string() + chars.as_str(),
					}
				}
				_ => first.to_uppercase().collect::<String>() + chars.as_str(),
			}
		}
	}
}

fn get_initials(s: &str) -> String
{
	let mut chars = s.chars();
	match chars.next() {
		None => String::new(),
		Some(first) => {
			match &first {
				'z' | 'c' | 's'  => match chars.next() {
					None => first.to_string(),
					Some(second) => match &second {
						'h' => first.to_string() + "h",
						_  => first.to_string(),
					}
				}
				_ => first.to_string()
			}
		}
	}
}

pub(super)  fn pinyin_fast_to_string<F> (input: &str, capitalize: CnCapitalize, operation : F  ) -> String
where F: Fn(Pinyin) -> String
{
	input.to_pinyin().flatten().map(|x| capitalize.from(&operation(x))).collect()
}

use itertools::Itertools;
use std::collections::HashSet;
#[allow(dead_code)]
pub(super)  fn pinyin_multi_to_vec<F> (input: &str, capitalize: CnCapitalize, operation : F ) -> Vec<String>
where F: Fn(Pinyin) -> String
{
	let choices: Vec<Vec<String>> = input
	.to_pinyin_multi()
	.flatten()
	.map(|multi| {
		// 这里先 collect 成 HashSet 去重，然后立刻转为 Vec
		multi.into_iter()
		.map(|p| capitalize.from(&operation(p)))
		.collect::<HashSet<_>>() // 去重
		.into_iter()             // 这里的迭代器仍然不支持 Clone...
		.collect::<Vec<_>>()     // ...但转成 Vec 后，Vec 的迭代器就支持 Clone 了
	}).collect();
	let results: Vec<String> = choices
	.into_iter()
	.multi_cartesian_product() // 核心逻辑：生成所有可能的组合
	.map(|combination| combination.join("")) // 将每种组合连成字符串
	.collect();
	return results;
}

#[allow(dead_code)]
pub(super)  fn pinyin_multi_to_hashset<F> (input: &str, capitalize: CnCapitalize, operation : F ) -> HashSet<String>
where F: Fn(Pinyin) -> String
{
	let choices: Vec<Vec<String>> = input
	.to_pinyin_multi()
	.flatten()
	.map(|multi| {
		multi.into_iter().map(|p| capitalize.from(&operation(p))).collect()
	}).collect();
	let results: HashSet<String> = choices
	.into_iter()
	.multi_cartesian_product() // 核心逻辑：生成所有可能的组合
	.map(|combination| combination.join("")) // 将每种组合连成字符串
	.collect();
	return results;
}
