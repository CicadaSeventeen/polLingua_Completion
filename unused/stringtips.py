def string_get_initials(s):
    return ''.join(word[0] for word in s.split())

def string_get_initials_lowered(s):
    return ''.join(word[0].lower() for word in s.split())

def string_get_initials_uppered(s):
    return ''.join(word[0].upper() for word in s.split())

def string_remove_lower(s):
    return ''.join(c for c in s if not c.islower())

def string_remain_upper(s):
    return ''.join(c for c in s if c.isupper())

