use hangeul::*;
use serde::{Deserialize, Serialize};

pub(super) fn hangeul_crate (input: &str) -> String
{
	let mut tmp_str = String::new();
	for char_item in input.chars() {
		if is_syllable(char_item as u32) {
			tmp_str.push(get_choseong(&char_item).unwrap());
			tmp_str.push(get_jungseong(&char_item).unwrap());
			if has_jongseong(&char_item).unwrap() {
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

pub(super) fn hangeul_crate_capitalize_choseong (input: &str) -> String
{
	let mut tmp_str = String::new();
	for char_item in input.chars() {
		if is_syllable(char_item as u32) {
			tmp_str = tmp_str +&(get_choseong(&char_item).unwrap().to_uppercase().to_string());
			tmp_str.push(get_jungseong(&char_item).unwrap());
			if has_jongseong(&char_item).unwrap() {
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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum KoCapitalize {
	Choseong,
	FirstLetter,
	All,
	No,
}

impl KoCapitalize {
	pub fn exec(&self, input: &str) -> String
	{
		match self {
			KoCapitalize::Choseong => hangeul_crate_capitalize_choseong(input),
			KoCapitalize::No => hangeul_crate(input),
			KoCapitalize::All => hangeul_crate(input).to_uppercase(),
			KoCapitalize::FirstLetter => capitalize_first(&hangeul_crate(input)),
		}
	}
}

fn capitalize_first(s: &str) -> String {
	let mut chars = s.chars();
	match chars.next() {
		None => String::new(),
		Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
	}
}
