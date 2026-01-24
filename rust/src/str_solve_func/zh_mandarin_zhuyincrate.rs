use pinyin::Pinyin;
use pinyin_zhuyin::*;
use crate::str_solve_func::zh_mandarin_pinyincrate::*;
use super::filter::*;

fn operation_zhuyin_normal(pinyin: Pinyin) -> String
{
	let tmp_str = pinyin.with_tone_num_end();
	encode_zhuyin(tmp_str).unwrap_or(encode_zhuyin(&(tmp_str.to_owned() + "5")).unwrap())
}

fn operation_zhuyin_strip(pinyin: Pinyin) -> String
{
	let tmp_str = pinyin.with_tone_num_end();
	let tmp_str = encode_zhuyin(tmp_str).unwrap_or(encode_zhuyin(&(tmp_str.to_owned() + "5")).unwrap());
	tmp_str.chars().filter(|&c| !matches!(c, 'ˊ' | 'ˇ' | 'ˋ' | '˙')).collect()
}

fn get_operation_zhuyin(strip :bool) -> fn(Pinyin) -> String
{
	let func = if strip == false {
		operation_zhuyin_normal
	} else {
		operation_zhuyin_strip
	};
	return func;
}

pub(super) fn hanzi_to_zhuyin_list(hanzi :&str, strip :bool, heteronym :bool) ->  Vec<String>
{
	let operation = get_operation_zhuyin(strip);
	if heteronym == true {
		pinyin_multi_to_hashset(hanzi, CnCapitalize::No, operation).into_iter().collect::<Vec<String>>()
	} else {
		vec![pinyin_fast_to_string(hanzi, CnCapitalize::No, operation)]
	}
}

pub(super) fn filtered_hanzi_to_zhuyin_list(hanzi :&str, strip :bool, heteronym :bool)  ->  Vec<String>
{
	let func = |x: &str| hanzi_to_zhuyin_list(x, strip, heteronym) ;
	exec_func_on_matched(hanzi, is_hanzi, func)
}
