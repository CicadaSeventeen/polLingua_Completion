import completion as cmplt
import sys
try:
    arg1 = sys.argv[1]
except IndexError:
    arg1 = ""
try:
    arg2 = sys.argv[2]
except IndexError:
    arg2 = "."
print(cmplt.main(target=arg1,path=arg2,mode="equal"))
