try:
    import anyascii
    Converter_Unicode = "anyascii"
except ImportError:
    try:
        import unidecode
        Converter_Unicode = "unidecode"
    except ImportError:
        Converter_Unicode = "none"
try:
    import pypinyin
    Converter_Chinese = "pypinyin"
except ImportError:
    Converter_Chinese = Converter_Unicode

def find_non_ascii_strings(path):
    results = []
    for entry in pathlib.Path(path).iterdir():
        string = entry.string
        try:
            string.encode('ascii')
        except UnicodeEncodeError:
            results.append(string)
    return results

print(Converter_Chinese,Converter_Unicode)

