from pypinyin import pinyin,Style
import unidecode as udc
import anyascii as asc

simplify = lambda l: list(set(l))
# def speial converter and alias
identity = lambda x: x
remove = lambda x: ['']
upper = lambda x: [ c.upper() for c in x ]
lower = lambda x: [ c.lower() for c in x ]
capitalize = lambda x: [ c.capitalize() for c in x ]
title = lambda x: [ c.title() for c in x ]
first_letter_unicode = lambda x: [ word[0] for word in x.split() ]
first_letter_unicode_upper = lambda x: [ word[0].upper() for word in x.split() ]
first_letter_unicode_lower = lambda x: [ word[0].lower() for word in x.split() ]
full = "full"
Full = "Full"
FUll = "FUll"
FuLl = "FuLl"
FULL = "FULL"
first_letter = "first_letter"
First_Letter = "First_Letter"
FIRST_LETTER = "FIRST_LETTER"
# for unicode
both = "both"
unidecode = "unidecode"
anyascii = "anyascii"
# for Chinese
initials = "initials"
INITIALS = "INITIALS"
Initials = "Initials"
initials_capitalize = lambda x: [ c.capitalize().replace('Ch', 'CH', 1).replace('Zh', 'ZH', 1).replace('Sh', 'SH', 1) for c in x]

def cartesian_product(list):
    import itertools
    return [''.join(comb) for comb in itertools.product(*list)]

def list_convert_unicode(string,converter=identity,converter_other=identity,converter_lib=None,extra_arg=both):
    if isinstance(converter, list):
        converter_other = converter[1:]
        converter = converter[0]
    if isinstance(extra_arg,list):
        converter_lib = extra_arg[0]
        extra_arg = extra_arg[1:]
    if converter == full:
        converter = lambda x: x.lower()
    elif converter == first_letter:
        converter = first_letter_unicode_lower
    elif converter == FIRST_LETTER:
        converter = first_letter_unicode_upper
    elif converter == First_Letter:
        converter = first_letter_unicode
    elif converter == FULL:
        converter = lambda x: x.upper()
    elif converter == FuLl:
        converter = identity
    elif converter == Full:
        converter = lambda x: x.title()
    if converter_lib == None:
        converter_lib = extra_arg
    if converter_lib == both:
        l = simplify([ converter(asc.anyascii(string)) , converter(udc.unidecode(string)) ])
    elif converter_lib == anyascii:
        l = [ converter(asc.anyascii(string)) ]
    elif converter_lib == unidecode:
        l = [ converter(udc.unidecode(string)) ]
# todo: 支持进一步的字符串处理，如ascii码（英文）
    else:
        raise ValueError
    return l


def list_convert_pinyin(string,converter_chinese=None,converter_other=identity,heteronym="auto",style=None,strict=False,converter=None,extra_arg=None):
    if isinstance(converter, list):
        converter_other = converter[1]
        converter = converter[0]
    if isinstance(extra_arg,list):
        heteronym = extra_arg[0]
        extra_arg = extra_arg[1:]
    if heteronym == None:
        heteronym = extra_arg
    if heteronym == "auto":
        heteronym = False
    elif heteronym == "off":
        heteronym = False
        string = list(string)
    elif heteronym == "on":
        heteronym = True
    elif heteronym == "all":
        heteronym = True
        string = list(string)
    else:
        raise ValueError
    if converter_chinese == None:
        converter_chinese = converter
    if style == None:
        if converter_chinese == full:
            style = Style.NORMAL
            converter_chinese = identity
        elif converter_chinese == first_letter:
            style = Style.FIRST_LETTER
            converter_chinese = identity
        elif converter_chinese == initials:
            style = Style.INITIALS
            converter_chinese = identity
        elif converter_chinese == Full:
            style = Style.NORMAL
            converter_chinese = capitalize
        elif converter_chinese == FULL:
            style = Style.NORMAL
            converter_chinese = upper
        elif converter_chinese == FUll:
            style = Style.NORMAL
            converter_chinese = initials_capitalize
        elif converter_chinese == FIRST_LETTER:
            style = Style.FIRST_LETTER
            converter_chinese = upper
        elif converter_chinese == Initials:
            style = Style.INITIALS
            converter_chinese = capitalize
        elif converter_chinese == INITIALS:
            style = Style.INITIALS
            converter_chinese = upper
        else:
            style = Style.NORMAL
            converter_chinese = identity
    double_list_1 = pinyin(string,style=style,heteronym=heteronym,strict=strict)
    double_list_2 = pinyin(string,style=style,heteronym=heteronym,strict=strict,errors="replace")
    double_list = []
    string_tmp = ""
    for count in range(0,len(double_list_1)):
        if double_list_1[count] == double_list_2[count]:
            #中文
            if string_tmp != "":
                double_list.append(list_convert_unicode(string_tmp,converter=converter_other,extra_arg=extra_arg))
                string_tmp = ""
            s=simplify(converter_chinese(double_list_1[count]))
            double_list.append(s)
        else:
            #非中文
            string_tmp = string_tmp + double_list_1[count][0]
    if string_tmp != "":
        double_list.append(list_convert_unicode(string_tmp,converter=converter_other,extra_arg=extra_arg))
    return double_list

def string_convert_pinyin(string,converter_chinese=None,converter_other=identity,heteronym="auto",style=None,strict=False,converter=None,extra_arg=None):
    return cartesian_product(list_convert_pinyin(string,converter_chinese=converter_chinese,converter_other=converter_other,heteronym=heteronym,style=style,strict=strict,converter=converter,extra_arg=extra_arg))

