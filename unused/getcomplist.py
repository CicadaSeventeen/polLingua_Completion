import completion as cmplt
import sys
try:
    arg1 = sys.argv[1]
except IndexError:
    arg2 = "auto"
try:
    arg2 = sys.argv[2]
except IndexError:
    arg2 = "equal"
try:
    arg3 = sys.argv[3]
except IndexError:
    arg3 = ''
try:
    arg4 = sys.argv[4]
except IndexError:
    arg4 = '.'

if arg1 == "direct":
    print(cmplt.main(target=arg3,path=arg4,mode=arg2))
elif arg1 == "client":
