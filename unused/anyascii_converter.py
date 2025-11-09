import os
from anyascii import anyascii
import stringtips
# 默认情况：首字母大写，后面小写，没有声调，没有空格问题
# 日汉字会被识别为汉语普通话
# 阿拉伯语转化只包括辅音（理所当然）
def converter_unicode_string(string):
    return anyascii(string)

def converter_unicode(list):
    return [ converter_unicode_string(string) for string in list ]

converter_chinese_string = converter_unicode_string

def converter_chinese(list):
    return [ converter_chinese_string(string) for string in list ]

converter_chinese_list = converter_chinese
converter_unicode_list = converter_unicode

