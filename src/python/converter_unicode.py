import unidecode as udc
import anyascii as asc
def unidecode(string):
	if isinstance(string,str):
		return udc.unidecode(string)
	elif isinstance(string,list):
		return  [udc.unidecode(c) for c in string ]

def anyascii(string):
	if isinstance(string,str):
		return asc.anyascii(string)
	elif isinstance(string,list):
		return  [ asc.anyascii(c) for c in string ]
