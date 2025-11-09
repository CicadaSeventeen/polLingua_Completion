import os
from converter_filter import filter_ascii_core, filter_hanzi_core

def list_filename(path):
    COMPLETION_FILE_TYPE = os.environ.get("COMPLETION_FILE_TYPE","dir:file").split(":")
    COMPLETION_SHOW_HIDDEN = os.environ.get("COMPLETION_SHOW_HIDDEN","yes")
    if "dir" in COMPLETION_FILE_TYPE:
        if "file" in COMPLETION_FILE_TYPE:
            ans = os.listdir(path)
        else:
            ans = [item for item in os.listdir(path) if os.path.isdir(os.path.join(path, item))]
    elif "file" in COMPLETION_FILE_TYPE:
        ans = [item for item in os.listdir(path) if os.path.isfile(os.path.join(path, item))]
    else:
        ans = []
    if COMPLETION_SHOW_HIDDEN == "no":
        ans = [item for item in ans if not item.startswith('.')]
    return ans

#list_filename = os.listdir
listdir = list_filename

def listdir_ascii_or_not(path):
    return filter_ascii_core(list_filename(path))

def listdir_chinese_or_not(path):
    return filter_hanzi_core(list_filename(path))

def listdir_ascii_and_chinese_or_not(path):
    [ ascii_list , non_ascii_list ] = filter_ascii_core(list_filename(path))
    return [ascii_list] + filter_hanzi_core(non_ascii_list)
# [ Ascii, Chinese, Other non-Ascii ]
# 1、目录、文件或全部
# 2、隐藏文件？
# 3、中文、非ascii、ascii
# ……
