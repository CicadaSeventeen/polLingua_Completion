import listdir_convert
import os
#import shlex

def dict_reverse_lookup(d, target):
    result = []
    for key, value_list in d.items():
        if target in value_list:
            result.append(key)
    return result

def dict_reverse_lookup_startswith(d, target):
    result = []
    for key, value_list in d.items():
        if any(s.startswith(target) for s in value_list):
            result.append(key)
    return result

def dict_reverse_lookup_case_insensitive(d, target):
    result = []
    for key, value_list in d.items():
        if target.lower() in [ s.lower() for s in value_list ]:
            result.append(key)
    return result

def dict_reverse_lookup_case_insensitive_startswith(d, target):
    result = []
    for key, value_list in d.items():
        if any(s2.startswith(target.lower()) for s2 in [ s.lower() for s in value_list ]):
            result.append(key)
    return result

def get_comp_list(target,path="."):
    mode = os.environ.get("COMPLETION_FILENAME_MATCH_MODE","equal")
    dict_listdir = listdir_convert.main(path)
    if mode == "equal":
        return dict_reverse_lookup(dict_listdir,target)
    elif mode == "startswith":
        return dict_reverse_lookup_startswith(dict_listdir,target)
    else:
        return []

def main(target,path='.'):
    quote = os.environ.get("COMPLETION_STRING_QUOTE_MODE","unescape")
    if quote == "unescape":
        answer = "\n".join(word for word in get_comp_list(target,path))
    elif quote == "quote":
        import shlex
        answer = "\n".join(shlex.quote(word) for word in get_comp_list(target,path))
    else:
        anser = ''
    return answer

if __name__ == "__main__":
    main(target='',path='.')
