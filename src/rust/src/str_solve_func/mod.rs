use serde::{Deserialize, Serialize};
use enum_dispatch::enum_dispatch;

pub mod filter;
use crate::str_solve_func::filter::*;

pub mod basic;
use crate::str_solve_func::basic::*;

pub mod zh;
pub mod zh_mandarin_pinyincrate;
pub mod zh_mandarin_zhuyincrate;
use crate::str_solve_func::zh::*;

pub mod ko;
pub mod ko_hengeulcrate;
use crate::str_solve_func::ko::*;

pub mod jp;
pub mod jp_mecab;
pub mod jp_wanakana;
use crate::str_solve_func::jp::*;

#[enum_dispatch]
pub trait StrSolveFuncExec {
    fn exec(&self, input: &str) -> Vec<String>;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "func", content = "args", rename_all = "snake_case")]
#[enum_dispatch(StrSolveFuncExec)]
pub enum StrSolveFunc {
	// Root
	Root(StrSolveFuncRoot),

	// Basic & Trivial
	Identity(StrSolveFuncIdentity),
	Affix(StrSolveFuncAffix),
	Unicode(StrSolveFuncUnicodeToAsciiAll),
	UnicodeAdvanced(StrSolveFuncUnicodeToAscii),
	//Anyascii(StrSolveFuncAnyAscii),
	//Unidecode(StrSolveFuncUnidecode),
	//Deunicode(StrSolveFuncDeunicode),

	// Filter
	Filter(StrSolveFuncFilter),

	// Chinese Mandarin (Putonghua)
	ZhHanzi(StrSolveFuncZhMandarin),
	ZhHanziPinyin(StrSolveFuncCnPinyin),
	ZhHanziZhuyin(StrSolveFuncZhuyin),

	// Japanese
	JpAll(StrSolveFuncJpAll),
	JpKanjiAndKana(StrSolveFuncJp),
	JpKana(StrSolveFuncJpKana),

	// Korean
	KoHangeul(StrSolveFuncHangeul),
	KoJamo(StrSolveFuncJamo),

	Other(StrSolveFuncTodo),
}


