use std::sync::{ Mutex, OnceLock};
//use serde::{Deserialize, Serialize};
use mecab::Tagger;
use std::collections::HashSet;
use crate::env::path::get_tmpfile_jp_dict_path;

struct GuardTagger(Tagger);
unsafe impl Send for GuardTagger {}

static GLOBAL_TAGGER: OnceLock<Mutex<GuardTagger>> = OnceLock::new();

fn get_guard_tagger() -> &'static Mutex<GuardTagger> {
	GLOBAL_TAGGER.get_or_init(|| {
		let tagger = Tagger::new("-O yomi".to_string() + " -d " + get_tmpfile_jp_dict_path().to_str().unwrap() + " -r "  +  get_tmpfile_jp_dict_path().to_str().unwrap());
		Mutex::new(GuardTagger(tagger))
	})
}

pub(super) fn mecab_nbest(nbest: u8, input :&str) -> Vec<String>
{
	let tagger = &mut get_guard_tagger().lock().unwrap().0;
	let mut tmp_str_set: HashSet<String> = HashSet::new();
	tagger.parse_nbest_init(input);
	for _i in 0..nbest {
		if let Some(res) = tagger.next() {
			tmp_str_set.insert(res);
		}
		else {
			break;
		}
	}
	tmp_str_set.into_iter().map(|x| x.trim().to_string()).collect::<Vec<_>>()
}

pub(super) fn mecab_fast_to_string(input :&str) -> String
{
	let tagger = &mut get_guard_tagger().lock().unwrap().0;
	tagger.parse_str(input).trim().to_string()
}

