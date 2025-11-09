import os
import sys
import re
import inspect
import converter as cvrt

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
	if os.environ.get("COMPLETION_CONVERTER_ARGUMENT_LIST"):
		for tmp_str in  os.environ.get("COMPLETION_CONVERTER_ARGUMENT_LIST").split(":"):
			all_converter_argv_list.append(tmp_str.strip().strip("{").strip("}").strip().split(","))
	#print(all_converter_list)
	#print(all_converter_argv_list)
	if  len(all_converter_list) == 1 and len(all_converter_argv_list) ==0:
		all_converter_argv_list.append([""])
	if len(all_converter_list) != len(all_converter_argv_list):
		print("Error: COMPLETION_CONVERTER_LIST and COMPLETION_CONVERTER_ARGUMENT_LIST length not consistant ",file=sys.stderr)
		exit(1)
	for count in range(0, len(all_converter_list)):
		ret = ret + run_list_converter(string, converter_list = all_converter_list[count], converter_argv_list = all_converter_argv_list[count])
	return ret
