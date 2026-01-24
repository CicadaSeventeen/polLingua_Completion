use hangeul::*;
use serde::{Deserialize, Serialize};
use crate::str_solve_func::basic::unicode_to_ascii;

fn hangeul_to_jamo_core (input: &str, choseong: bool, jungseong: bool, jongseong: bool) -> String
{
	let mut tmp_str = String::new();
	for char_item in input.chars() {
		if is_syllable(char_item as u32) {
			if choseong == true {
				tmp_str.push(get_choseong(&char_item).unwrap());
			}
			if jungseong == true {
				tmp_str.push(get_jungseong(&char_item).unwrap());
			}
			if jongseong == true && has_jongseong(&char_item).unwrap() {
				tmp_str.push(get_jongseong(&char_item).unwrap());
			}
		}
		else {
			tmp_str.push(char_item);
		}
	}
	let tmp_str = tmp_str;
	tmp_str
}

/*
#[allow(dead_code)]
fn hangeul_to_ascii_core<F> (input: &str, choseong: bool, jungseong: bool, jongseong: bool, operation_on_choseong: F) -> Vec<String>
where
	F: Fn(char) -> String,
{
	let mut tmp_str = String::new();
	for char_item in input.chars() {
		if is_syllable(char_item as u32) {
			if choseong == true {
				let item_str = operation_on_choseong(get_choseong(&char_item).unwrap());
				if item_str != "" {
					tmp_str  += &(item_str);
				}
			}
			if jungseong == true {
				tmp_str.push(get_jungseong(&char_item).unwrap());
			}
			if jongseong == true && has_jongseong(&char_item).unwrap() {
				tmp_str.push(get_jongseong(&char_item).unwrap());
			}
		}
		else {
			tmp_str.push(char_item);
		}
	}
	unicode_to_ascii(&tmp_str)
}*/

use itertools::Itertools;
/*
fn hangeul_to_ascii_first_letter<F> (input: &str, capitalize: bool) -> Vec<String>
{
	let mut choices Vec<Vec<String>> = Vec::new();
	for char_item in input.chars() {
		if is_syllable(char_item as u32) {
			let mut item_str = get_choseong(&char_item).unwrap().to_string();
			if item_str == "" {
				item_str += &get_jungseong(&char_item).unwrap().to_string()
			}
			if capitalize == true {
				choices.push(unicode_to_ascii(item_str).iter().to_uppercase().collect());
			} else {
				choices.push(unicode_to_ascii(item_str))
			}
		}
		else {
			choices.push(vec![char_item]);
		}
	}
	let results: Vec<String> = choices
	.into_iter()
	.multi_cartesian_product() // 核心逻辑：生成所有可能的组合
	.flat_map(|combination| combination.join("")) // 将每种组合连成字符串
	.collect();
	return results;
}*/

fn hangeul_to_ascii_core_cartesian<F1,F2>  (input: &str, choseong: bool, jungseong: bool, jongseong: bool, operation_on_choseong: F1, operation_on_word: F2) -> Vec<String>
where
F1: Fn(String) -> String,
F2: Fn(String) -> String
{
	let mut choices: Vec<Vec<String>> = Vec::new();
	for char_item in input.chars() {
		if is_syllable(char_item as u32) {
			let mut tmp_choices: Vec<Vec<String>> = Vec::new();
			if choseong == true {
				let item_tmp = unicode_to_ascii(&get_choseong(&char_item).unwrap().to_string());
				if item_tmp != [""] {
					tmp_choices.push(item_tmp.into_iter().map(|x| operation_on_choseong(x)).collect());
				}
			}
			if jungseong == true {
				tmp_choices.push(unicode_to_ascii(&get_jungseong(&char_item).unwrap().to_string()));
			}
			if jongseong == true && has_jongseong(&char_item).unwrap() {
				tmp_choices.push(unicode_to_ascii(&get_jongseong(&char_item).unwrap().to_string()));
			}
			let word : Vec<String> = tmp_choices	.into_iter()
			.multi_cartesian_product()
			.map(|combination| operation_on_word(combination.join("")))
			.collect();
			choices.push(word);
		}
		else if is_hangeul(char_item as u32) {
			choices.push(unicode_to_ascii(&operation_on_word(char_item.to_string())))
		}
		else {
			choices.push(vec![char_item.to_string()]);
		}
	}
	let results: Vec<String> = choices
	.into_iter()
	.multi_cartesian_product() // 核心逻辑：生成所有可能的组合
	.map(|combination| combination.join("")) // 将每种组合连成字符串
	.collect();
	return results;
}

pub(super) fn hangeul_to_ascii (input: &str, format: KoFormat, capitalize: KoCapitalize) -> Vec<String>
{
	match format {
		KoFormat::Full => match capitalize {
			KoCapitalize::No => hangeul_to_ascii_core_cartesian(input,true,true,true,|x| x,|x| x),
			KoCapitalize::All => hangeul_to_ascii_core_cartesian(input,true,true,true,|x| x,|x|  x.to_uppercase()),
			KoCapitalize::FirstLetter => hangeul_to_ascii_core_cartesian(input,true,true,true,|x| x,|x| capitalize_first(&x)),
			KoCapitalize::Choseong => hangeul_to_ascii_core_cartesian(input,true,true,true,|x| x.to_uppercase(),|x| x),
		},
		KoFormat::Choseong => match capitalize {
			KoCapitalize::No => hangeul_to_ascii_core_cartesian(input,true,false,false,|x| x, |x| x),
			KoCapitalize::FirstLetter => hangeul_to_ascii_core_cartesian(input,true,false,false,|x| x.chars().next().unwrap().to_string(),|x| x),
			KoCapitalize::Choseong | KoCapitalize::All => hangeul_to_ascii_core_cartesian(input,true,false,false,|x|  x.to_uppercase(), |x| x),
		},
		KoFormat::FirstLetter => match capitalize {
			KoCapitalize::No => hangeul_to_ascii_core_cartesian(input,true,true,false,|x| x,|x| x.chars().next().unwrap().to_string()),
			KoCapitalize::FirstLetter | KoCapitalize::Choseong | KoCapitalize::All => hangeul_to_ascii_core_cartesian(input,true,true,false,|x| x,|x| x.chars().next().unwrap().to_string().to_uppercase())
		},
	}
}

pub(super) fn hangeul_to_jamo (input: &str, format: KoFormat) -> String
{
	match format {
		KoFormat::Full => hangeul_to_jamo_core(input,true,true,true),
		_ => hangeul_to_jamo_core(input,true,false,false),
	}
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum KoCapitalize {
	Choseong,
	FirstLetter,
	All,
	No,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum KoFormat {
	Choseong,
	FirstLetter,
	Full,
}

fn capitalize_first(s: &str) -> String {
	let mut chars = s.chars();
	match chars.next() {
		None => String::new(),
		Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
	}
}
