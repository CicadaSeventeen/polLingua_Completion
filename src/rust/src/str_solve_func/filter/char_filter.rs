pub use unicode_blocks as ub;
use hangeul::is_syllable;
use hangeul::is_jamo as is_jamo_u32;

pub fn is_ascii(c: char) -> bool {
    c.is_ascii()
}

pub fn is_unicode(c: char) -> bool {
    !c.is_ascii()
}

pub fn is_cjk(c: char) -> bool {
	ub::is_cjk(c)
}

pub fn is_hangeul(c: char) -> bool {
	is_syllable(c as u32)
}

pub fn is_jamo(c: char) -> bool {
	is_jamo_u32(c as u32)
}

pub fn is_ko(c: char) -> bool {
	is_hangeul(c) || is_jamo(c)
}

pub fn is_hanzi(c: char) -> bool {
	let tmp = ub::find_unicode_block(c);
	if tmp == Some(ub::CJK_UNIFIED_IDEOGRAPHS) {
		return true;
	} else if tmp == Some(ub::CJK_UNIFIED_IDEOGRAPHS_EXTENSION_A) {
		return true;
	} else if tmp == Some(ub::CJK_UNIFIED_IDEOGRAPHS_EXTENSION_B) {
		return true;
	} else if tmp == Some(ub::CJK_UNIFIED_IDEOGRAPHS_EXTENSION_C) {
		return true;
	} else if tmp == Some(ub::CJK_UNIFIED_IDEOGRAPHS_EXTENSION_D) {
		return true;
	} else if tmp == Some(ub::CJK_UNIFIED_IDEOGRAPHS_EXTENSION_E) {
		return true;
	} else if tmp == Some(ub::CJK_UNIFIED_IDEOGRAPHS_EXTENSION_F) {
		return true;
	} else if tmp == Some(ub::CJK_UNIFIED_IDEOGRAPHS_EXTENSION_G) {
		return true;
	} else if tmp == Some(ub::CJK_UNIFIED_IDEOGRAPHS_EXTENSION_H) {
		return true;
	} else if tmp == Some(ub::CJK_UNIFIED_IDEOGRAPHS_EXTENSION_I) {
		return true;
	} else if tmp == Some(ub::CJK_COMPATIBILITY_IDEOGRAPHS_SUPPLEMENT ) {
		return true;
	} else {
		return false;
	}
}

pub fn is_zh(c: char) -> bool {
	is_hanzi(c)
}

pub fn is_hiragana(c: char) -> bool {
	if ub::find_unicode_block(c) == Some(ub::HIRAGANA) {
		return true;
	} else {
		return false;
	}
}

pub fn is_katakana(c :char) -> bool {
	let tmp = ub::find_unicode_block(c);
	if tmp  == Some(ub::KATAKANA) {
		return true;
	} else if tmp == Some(ub::KATAKANA_PHONETIC_EXTENSIONS) {
		return true;
	} else {
		return false;
	}
}

pub fn is_kana(c :char) -> bool {
	if is_katakana(c) || is_hiragana(c) {
		return true;
	} else {
		let tmp = ub::find_unicode_block(c);
		if tmp == Some(ub::KANA_SUPPLEMENT) || tmp == Some(ub::KANA_EXTENDED_A) || tmp == Some(ub::KANA_EXTENDED_B) || tmp == Some(ub::SMALL_KANA_EXTENSION) {
			return true;
		}
		else {
			return false;
		}
	}
}

pub fn is_jp(c: char) -> bool {
	is_kana(c) || is_hanzi(c)
}

pub fn is_latin(c: char) -> bool {
	let tmp = ub::find_unicode_block(c);
	if c.is_ascii() {
		return true;
	}	else if tmp == Some(ub::BASIC_LATIN) {
		return true;
	} else if tmp == Some(ub::	LATIN_1_SUPPLEMENT) {
		return true;
	} else if tmp == Some(ub::	LATIN_EXTENDED_A) {
		return true;
	} else if tmp == Some(ub::	LATIN_EXTENDED_ADDITIONAL) {
		return true;
	} else if tmp == Some(ub::	LATIN_EXTENDED_B) {
		return true;
	} else if tmp == Some(ub::	LATIN_EXTENDED_C) {
		return true;
	} else if tmp == Some(ub::	LATIN_EXTENDED_D) {
		return true;
	} else if tmp == Some(ub::	LATIN_EXTENDED_E) {
		return true;
	} else if tmp == Some(ub::	LATIN_EXTENDED_F) {
		return true;
	} else if tmp == Some(ub::	LATIN_EXTENDED_G) {
		return true;
	} else {
		return false;
	}
}

pub fn is_greek(c: char) -> bool {
	let tmp = ub::find_unicode_block(c);
	if tmp == Some(ub::GREEK_AND_COPTIC) {
			return true;
		} else if tmp == Some(ub::GREEK_EXTENDED) {
			return true;
		} else {
			return false;
		}
}

pub fn is_cyrillic(c: char) -> bool {
	let tmp = ub::find_unicode_block(c);
	if tmp == Some(ub::CYRILLIC) {
		return true;
		} else if tmp == Some(ub::CYRILLIC_EXTENDED_A) {
			return true;
		} else if tmp == Some(ub::CYRILLIC_EXTENDED_B) {
			return true;
		} else if tmp == Some(ub::CYRILLIC_EXTENDED_C) {
			return true;
		} else if tmp == Some(ub::CYRILLIC_EXTENDED_D) {
			return true;
		} else if tmp == Some(ub::CYRILLIC_SUPPLEMENT) {
			return true;
		} else {
			return false;
		}
}

