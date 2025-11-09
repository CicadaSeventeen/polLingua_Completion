# 去除重复结果
simplify = lambda l: list(set(l))
# 大小写处理
identity = lambda x: x

def remove(string):
	if isinstance(string,str):
		return ""
	else:
		return [""]

def upper(string):
	if isinstance(string,str):
		return string.upper()
	elif isinstance(string,list):
		return [x.upper() for x in string]

def lower(string):
	if isinstance(string,str):
		return string.lower()
	elif isinstance(string,list):
		return [x.lower() for x in string]

def capitalize(string):
	if isinstance(string,str):
		return string.capitalize()
	elif isinstance(string,list):
		return [x.capitalize() for x in string]

def capitalize_title(string):
	if isinstance(string,str):
		return string.title()
	elif isinstance(string,list):
		return [x.title() for x in string]

def first_letter(string):
	if isinstance(string,str):
		return "".join(word[0] for word in string.split())
	elif isinstance(string,list):
		return ["".join(word[0] for word in x.split()) for x in string]

# 汉语拼音声母大写处理
def initials_capitalize(string):
	if isinstance(string,str):
		return  string.capitalize().replace('Ch', 'CH', 1).replace('Zh', 'ZH', 1).replace('Sh', 'SH', 1)
	elif isinstance(string,list):
		return  [ c.capitalize().replace('Ch', 'CH', 1).replace('Zh', 'ZH', 1).replace('Sh', 'SH', 1) for c in string ]

def initials_capitalize_title(string):
	if isinstance(string,str):
		return  string.title().replace('Ch', 'CH', 1).replace('Zh', 'ZH', 1).replace('Sh', 'SH', 1)
	elif isinstance(string,list):
		return  [ c.title().replace('Ch', 'CH', 1).replace('Zh', 'ZH', 1).replace('Sh', 'SH', 1) for c in string ]
