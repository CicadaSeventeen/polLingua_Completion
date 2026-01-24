use std::borrow::Cow;
use itertools::Itertools;
//use std::collections::HashSet;

pub fn contains_category<F>(s: &str, predicate: F) -> bool
where
F: Fn(char) -> bool,
{
	s.chars().any(predicate)
}

/// 检查字符串是否完全由满足 predicates 条件的字符组成
pub fn is_entirely_category<F>(s: &str, predicate: F) -> bool
where
F: Fn(char) -> bool,
{
	!s.is_empty() && s.chars().all(predicate)
}

#[derive(Debug, PartialEq, Clone)]
enum TextSlice<'a> {
	Matched(Cow<'a, str>),
	Unmatched(Cow<'a, str>),
}

impl<'a> TextSlice<'a> {
	// 返回引用，不转移所有权
	pub fn as_str(&self) -> &str {
		match self {
			TextSlice::Matched(s) | TextSlice::Unmatched(s) => s.as_ref(),
		}
	}
}

/// 1. 分割逻辑保持高效，初始全部是 Borrowed
fn split_by_category<'a, F>(s: &'a str, predicate: F) -> Vec<TextSlice<'a>>
where
F: Fn(char) -> bool,
{
	let mut segments = Vec::new();
	let mut iter = s.char_indices().peekable();

	while let Some((start_idx, first_char)) = iter.next() {
		let target_state = predicate(first_char);
		let mut end_idx = start_idx + first_char.len_utf8();

		while let Some(&(idx, c)) = iter.peek() {
			if predicate(c) == target_state {
				end_idx = idx + c.len_utf8();
				iter.next();
			} else {
				break;
			}
		}

		let slice = &s[start_idx..end_idx];
		if target_state {
			segments.push(TextSlice::Matched(Cow::Borrowed(slice)));
		} else {
			segments.push(TextSlice::Unmatched(Cow::Borrowed(slice)));
		}
	}
	segments
}

/// 2. 关键修改：operation 现在可以直接把 Cow 的所有权交给 TextSlice
#[allow(dead_code)]
fn exec_func_single_on_matched_to_segments<'a, F>(
	segments: Vec<TextSlice<'a>>,
	operation: F
) -> Vec<TextSlice<'a>>
where
F: Fn(&str) -> Cow<'a, str>,
{
	segments.into_iter().map(|x| {
		match x {
			TextSlice::Matched(s) => {
				// 这里 s 是 Cow，我们取其引用传给 operation
				// operation 返回的新 Cow 直接存入 Matched
				TextSlice::Matched(operation(s.as_ref()))
			},
			TextSlice::Unmatched(_) => x,
		}
	}).collect()
}

/// 3.笛卡尔积处理
pub fn exec_func_on_matched<F1, F2>(input: &str, predicate: F1, operation: F2) -> Vec<String>
where
F1: Fn(char) -> bool,
F2: Fn(&str) -> Vec<String>,
{
	let segments = split_by_category(input, predicate);
	let choices: Vec<Vec<String>> = segments.into_iter().map(|seg| match seg {
		TextSlice::Matched(s) => operation(s.as_ref()),
		TextSlice::Unmatched(s) => vec![s.into_owned()], // 转换为 String
	}).collect();

	choices
	.into_iter()
	.multi_cartesian_product()
	.map(|combination| combination.join(""))
	.collect()
}

#[allow(dead_code)]
fn get_split_list<'a>(segments: Vec<TextSlice<'a>>) -> (Vec<String>, Vec<String>) {
	let tmp: (Vec<TextSlice<'a>>, Vec<TextSlice<'a>>) = segments.into_iter().partition(|seg| {
		matches!(seg, TextSlice::Matched(_))
	});
	(
		tmp.0.iter().map(|x| x.as_str().to_string()).collect(),
		tmp.1.iter().map(|x| x.as_str().to_string()).collect()
	)
}

#[allow(dead_code)]
fn join_segments<'a>(segments: Vec<TextSlice<'a>>) -> String
{
	segments.into_iter().map(|seg| match seg {
		TextSlice::Matched(s) => s,
		TextSlice::Unmatched(s) => s,
	}).collect::<String>()
}
