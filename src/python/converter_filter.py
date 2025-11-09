import re
def filter_ascii_core(tmp_list):
#返回：ascii列表和非ascii列表
	if isinstance(tmp_list, list):
		non_ascii_list = []
		ascii_list = []
		for tmp_str in tmp_list:
			try:
				tmp_str.encode('ascii')
				ascii_list.append(tmp_str)
			except UnicodeEncodeError:
				non_ascii_list.append(tmp_str)
		return [ascii_list,non_ascii_list]
	elif isinstance(tmp_list, str):
		return filter_ascii_core([ tmp_list ])

def filter_hanzi_core(tmp_list):
	if isinstance(tmp_list, list):
		hanzi_list = []
		non_hanzi_list = []
		all_hanzi_list = []
		for tmp_str in tmp_list:
			if bool(re.search(r'[\u4e00-\u9fff]+', tmp_str)):
				hanzi_list.append(tmp_str)
			else:
				non_hanzi_list.append(tmp_str)
			if bool(re.fullmatch(r'[\u4e00-\u9fff]+', tmp_str.strip())):
				all_hanzi_list.append(tmp_str)
		return [all_hanzi_list, hanzi_list,non_hanzi_list]
	elif isinstance(tmp_list, str):
		return filter_hanzi_core([ tmp_list ])

def filter_all_ascii(string):
	return filter_ascii_core(string)[0]

def filter_no_ascii(string):
	return filter_ascii_core(string)[1]

filter_ascii = filter_no_ascii

def filter_include_hanzi(string):
	return filter_hanzi_core(string)[1]

def filter_all_hanzi(string):
	return filter_hanzi_core(string)[0]

def filter_no_hanzi(string):
	return filter_hanzi_core(string)[2]

filter_hanzi = filter_include_hanzi

filter_unicode = filter_no_ascii

filter_include_unicode = filter_no_ascii

filter_no_unicode = filter_all_ascii
