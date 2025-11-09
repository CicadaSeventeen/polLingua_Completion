import listdir
import converter as cvrt
import os
import copy
import sys
import re
import inspect

def run_converter(string_list, converter="identity", converter_argv=[]):
	if isinstance(string_list,str):
		string_list =  [string_list]
	argv_dict = dict()
	try:
		for tmp_argv in converter_argv:
			try:
				tmp_key, tmp_value = tmp_argv.split('=', 1)
			except ValueError:
				tmp_key = tmp_argv
				tmp_value = True
			argv_dict.update({tmp_key: tmp_value})
	finally:
		pass
	funct_convert = getattr(cvrt, converter)
	ret = []
	for tmp_str in string_list:
		if  len(inspect.signature(funct_convert).parameters) > 1:
			tmp_ret = funct_convert(tmp_str, argv_dict)
		else:
			tmp_ret =  funct_convert(tmp_str)
		if isinstance(tmp_ret,str):
			tmp_ret = [tmp_ret]
		ret = ret + tmp_ret
	return ret

def run_list_converter (string_list, converter_list=["identity"], converter_argv_list=[""]):
	if len(converter_list) != len(converter_argv_list):
		print("Error: COMPLETION_CONVERTER_LIST and COMPLETION_CONVERTER_ARGUMENT_LIST length not consistant ",file=sys.stderr)
		exit(1)
	tmp_converter = converter_list[-1]
	tmp_converter_argv = re.split(r'[@#%^&]+',converter_argv_list[-1])
	if len(converter_list)>1:
		converter_list_remain = converter_list[0:-1]
		converter_argv_list_remain = converter_argv_list[0:-1]
		return run_converter(run_list_converter(string_list, converter_list_remain, converter_argv_list_remain), tmp_converter, tmp_converter_argv)
	else:
		return run_converter(string_list, tmp_converter, tmp_converter_argv)

def run_all_converter(string):
	all_converter_list = []
	all_converter_argv_list = []
	ret = []
	if os.environ.get("COMPLETION_CONVERTER_LIST"):
		for tmp_str in  os.environ.get("COMPLETION_CONVERTER_LIST").split(":"):
			all_converter_list.append(tmp_str.strip().strip("{").strip("}").strip().split(","))
	else:
		all_converter_list= [["pypinyin_filtered","anyascii"],["pypinyin_filtered","anyascii"],["pypinyin_filtered","anyascii"],["pypinyin","anyascii"],["pypinyin","anyascii"],["pypinyin","anyascii"],["filter_no_hanzi","unidecode"],["filter_no_hanzi","anyascii"],["identity"]]
	if os.environ.get("COMPLETION_CONVERTER_ARGUMENT_LIST"):
		for tmp_str in  os.environ.get("COMPLETION_CONVERTER_ARGUMENT_LIST").split(":"):
			all_converter_argv_list.append(tmp_str.strip().strip("{").strip("}").strip().split(","))
	else:
		all_converter_argv_list = [["style=normal#filter=capitalize","none"],["style=first_letter#filter=capitalize","none"],["style=initials#filter=initials_capitalize","none"],["style=normal","none"],["style=first_letter","none"],["style=initials","none"],["none","none"],["none","none"],["none"]]
	if  len(all_converter_list) > len(all_converter_argv_list) :
		all_converter_argv_list.append([""])
	if len(all_converter_list) != len(all_converter_argv_list):
		print("Error: COMPLETION_CONVERTER_LIST and COMPLETION_CONVERTER_ARGUMENT_LIST length not consistant ",file=sys.stderr)
		exit(1)
	for count in range(0, len(all_converter_list)):
		ret = ret + run_list_converter(string, converter_list = all_converter_list[count], converter_argv_list = all_converter_argv_list[count])
	return cvrt.simplify(ret)

def merge_dict(dict1, dict2):
	result = dict()
	all_keys = set(dict1) | set(dict2)
	for key in all_keys:
		result[key] = cvrt.simplify(dict1.get(key, []) + dict2.get(key, []))
	return result

def lowercase_dict(d):
	return {
		key: [s.lower() for s in value]
		for key, value in d.items()
	}

def uppercase_dict(d):
	return {
		key: [s.upper() for s in value]
		for key, value in d.items()
	}

def convert_listdir(list_str_listdir):
	dict_converted = dict()
	for s in list_str_listdir:
		dict_converted[s] = run_all_converter(s)
	return dict_converted

def main(path="."):
	argv_ignore_ascii = os.environ.get("COMPLETION_INGORE_ASCII","no").lower()
	argv_case_insensitive = os.environ.get("COMPLETION_CASE_INSENSITIVE","no").lower()
	if argv_ignore_ascii == "yes":
		list_str_listdir = listdir.listdir_ascii_or_not(path=path)
	else:
		list_str_listdir = listdir.listdir(path=path)
	dict_listdir = convert_listdir(list_str_listdir)
	if argv_case_insensitive == "yes":
		return merge_dict(dict_listdir, lowercase_dict(dict_listdir))
	else:
		return dict_listdir

#todo 处理不转译拼音的情况

if __name__ == "__main__":
	try:
		arg1 = sys.argv[1]
	except IndexError:
		arg1 = "."
	print(main(arg1))
