import completion as cmplt
import sys
import os
try:
    arg1 = sys.argv[1]
except IndexError:
    arg1 = ""
try:
    arg2 = sys.argv[2]
except IndexError:
    arg2 = "."
path_tmp,target = os.path.split(arg1)
if os.path.isabs(arg1):
    path = path_tmp
else:
    path = arg2 + '/' + path_tmp
print(cmplt.main(target=target,path=path,mode="startswith"))
#匹配模式
#文件or目录or全部
#是否经过daemon
