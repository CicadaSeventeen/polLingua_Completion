import itertools
from pypinyin import pinyin,Style
import converter_base

def _cartesian_product(l):
    return [''.join(comb) for comb in itertools.product(*l)]

# py pinyin
def _pypinyin_core(string, argv_dict, argv_errors):
	argv_strict = False
	if "strict" in argv_dict:
		if argv_dict["strict"] not in ["No","no","False","false","FASLE","0",0,False]:
			argv_strict=True
	argv_heteronym = False
	if "heteronym" in argv_dict:
		if argv_dict["heteronym"] == "all":
			string = list(string)
			argv_heteronym = True
		elif argv_dict["heteronym"] == "on" or argv_dict["heteronym"] == "true":
			argv_heteronym = True
		elif argv_dict["heteronym"] == "off" or argv_dict["heteronym"] == "false":
			string = list(string)
			argv_heteronym = False
		else:
			# default: auto
			argv_heteronym = False
	argv_style = Style.NORMAL
	if "style" in argv_dict:
		if argv_dict["style"].lower() == "normal":
			argv_style = Style.NORMAL
		elif argv_dict["style"].lower() == "first_letter":
			argv_style = Style.FIRST_LETTER
		elif argv_dict["style"].lower() == "initials":
			argv_style = Style.INITIALS
		else:
			argv_style = Style.NORMAL
	return pinyin(string, heteronym = argv_heteronym, errors = argv_errors, style = argv_style)

def pypinyin(string, argv_dict):
	argv_tmp_errors = "default"
	if "errors" in argv_dict:
		argv_tmp_errors = argv_dict["errors"]
	return _cartesian_product(_pypinyin_core(string, argv_dict, argv_errors=argv_tmp_errors))

def pypinyin_filtered(string, argv_dict):
	ret_list = []
	ret_list_1 = []
	ret_list_2 = []
	ret_list_1 = _pypinyin_core(string, argv_dict, argv_errors="default")
	ret_list_2 = _pypinyin_core(string, argv_dict, argv_errors="replace")
	filter_funct = converter_base.identity
	if "filter"  in argv_dict:
		filter_funct = getattr(converter_base,argv_dict["filter"])
	for count in range(0,len(ret_list_1)):
		if ret_list_1[count] == ret_list_2[count]:
			#中文
			ret_list.append(filter_funct(ret_list_1[count]))
		else:
			#非中文
			ret_list.append(ret_list_1[count])
	return _cartesian_product(ret_list)
