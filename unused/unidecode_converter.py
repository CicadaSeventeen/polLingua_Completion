from unidecode import unidecode
# 默认情况：首字母大写，后面小写，没有声调，每个汉字后出现一个空格，包括最后一个字符
# 日语假名和韩语谚文没有这个问题，但日汉字会被识别为汉语普通话
# 阿拉伯语转化只包括辅音（理所当然）
def converter_unicode_string(string):
    return unidecode(string)

def converter_unicode(list):
    return [ converter_unicode_string(string) for string in list ]

def converter_chinese_string(string):
    return converter_unicode_string(converter_unicode_string(string)).replace(" ","")

def converter_chinese(list):
    return [ converter_chinese_string(string) for string in list ]

converter_chinese_list = converter_chinese
converter_unicode_list = converter_unicode
