from main import get_string,solve_arg
import os
os.environ['COMPLETION_STRING_QUOTE_MODE'] = 'quote'
os.environ['COMPLETION_FILENAME_MATCH_MODE'] = 'startswith'
COMPLETION_FILE_TYPE = os.environ.get("COMPLETION_FILE_TYPE","dir:file").split(":")

[run_mode,target,path] = solve_arg()
cmd_string = ':'
dir_string = "compadd -f -U -S '\\' -Q "
file_string = "compadd -f -U -S ' ' -Q "
if 'dir' in COMPLETION_FILE_TYPE:
    os.environ['COMPLETION_FILE_TYPE'] = 'dir'
    #print(os.environ.get("COMPLETION_FILE_TYPE"))
    return_string = get_string(run_mode=run_mode,target=target,path=path)
    word_list = return_string.split('\n')
    if any(word_list):
        cmd_string = cmd_string + ';' + dir_string + ' '.join(word_list)
if 'file' in COMPLETION_FILE_TYPE:
    os.environ['COMPLETION_FILE_TYPE'] = 'file'
    return_string = get_string(run_mode=run_mode,target=target,path=path)
    word_list = return_string.split('\n')
    if any(word_list):
        cmd_string = cmd_string + ';' + file_string + ' '.join(word_list)
print(cmd_string)

