#import pathlib
import os
try:
    from anyascii import anyascii
    Converter_Unicode = anyascii
except ImportError:
    try:
        from unidecode import unidecode
        Converter_Unicode = unidecode
    except ImportError:
        pass
try:
    from pypinyin import pinyin, lazy_pinyin, Style
    Converter_Chinese = lazy_pinyin
except ImportError:
    Converter_Chinese = Converter_Unicode

# f_d is alias for files_or/and_dirs
get_f_d = os.listdir
def get_non_ascii_f_d(path):
    results = []
    for string in os.listdir(path):
        try:
            string.encode('ascii')
        except UnicodeEncodeError:
            results.append(string)
    return results

nonAscii_f_d = get_non_ascii_f_d("/home/cic17")
all_f_d = get_f_d("/home/cic17")
ascii_ized_f_d = [ Converter_Unicode(string) for string in nonAscii_f_d ]
print(ascii_ized_f_d)
