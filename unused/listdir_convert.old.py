import listdir
import converter as cvtr
import os
import copy
import sys

simplify = lambda l: list(set(l))

def cartesian_product(list1,list2):
    import itertools
    return [list(pair) for pair in itertools.product(list1, list2)]

def merge_dict(dict1, dict2):
    result = dict()
    all_keys = set(dict1) | set(dict2)
    for key in all_keys:
        result[key] = simplify(dict1.get(key, []) + dict2.get(key, []))
    return result

def lowercase_dict(d):
    return {
        key: [s.lower() for s in value]
        for key, value in d.items()
    }

def converter_listdir(list_path,converter,extra_arg):
    dict_convert = dict()
    for s in list_path:
        dict_convert[s] = cvtr.string_convert(s,converter=converter,extra_arg=extra_arg)
    return dict_convert

def converter_list_by_each_converter(list_path,converter_list,extra_arg):
    result = dict()
    for c in converter_list:
        result = merge_dict(copy.deepcopy(result),converter_listdir(list_path,c,extra_arg))
    return result

def main(path="."):
    COMPLETION_INGORE_ASCII = os.environ.get("COMPLETION_INGORE_ASCII","no")
    str_COMPLETION_TYPE = os.environ.get("COMPLETION_TYPE","chinese:unicode")
    COMPLETION_LANGUAGE_TYPE = os.environ.get("COMPLETION_LANGUAGE_TYPE",str_COMPLETION_TYPE).split(":")
    COMPLETION_PINYIN_HETERONYM = os.environ.get("COMPLETION_PINYIN_HETERONYM","auto")
    COMPLETION_CONVERTER_PINYIN = os.environ.get("COMPLETION_CONVERTER_PINYIN","full:Full:FIRST_LETTER:first_letter").split(":")
    COMPLETION_CONVERTER_UNICODE = os.environ.get("COMPLETION_CONVERTER_UNICODE","FuLl").split(":")
    COMPLETION_UNICODE_LIB = os.environ.get("COMPLETION_UNICODE_LIB","both")
    COMPLETION_CASE_INSENSITIVE = os.environ.get("COMPLETION_CASE_INSENSITIVE","no")
    if COMPLETION_INGORE_ASCII.lower() == "yes":
        list_path = listdir.listdir_ascii_or_not(path=path)
    else:
        list_path = listdir.listdir(path=path)
    if "chinese" in ( string.lower() for string in COMPLETION_LANGUAGE_TYPE):
        if "unicode" in ( string.lower() for string in COMPLETION_LANGUAGE_TYPE):
            dict_listdir = converter_list_by_each_converter(list_path,cartesian_product(COMPLETION_CONVERTER_PINYIN,COMPLETION_CONVERTER_UNICODE),[COMPLETION_PINYIN_HETERONYM,COMPLETION_UNICODE_LIB])
        else:
            dict_listdir = converter_list_by_each_converter(list_path,[COMPLETION_CONVERTER_PINYIN,None],[COMPLETION_PINYIN_HETERONYM,"both"])
    if COMPLETION_CASE_INSENSITIVE.lower() == "yes":
        dict_listdir = merge_dict(copy.deepcopy(dict_listdir),lowercase_dict(copy.deepcopy(dict_listdir)))
    return dict_listdir

#todo 处理不转译拼音的情况

if __name__ == "__main__":
    try:
        arg1 = sys.argv[1]
    except IndexError:
        arg1 = "."
    main(arg1)
