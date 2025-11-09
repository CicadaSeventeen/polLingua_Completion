import sys
import os
args = sys.argv[1:]
if len(args) >2:
    run_mode = args[0]
    arg_target = args[1]
    arg_path = args[2]
else:
    arg_target = args[0]
    arg_path = args[1]
path_tmp,target = os.path.split(arg_target)
if os.path.isabs(path_tmp):
    path = path_tmp
else:
    path = arg_path + '/' + path_tmp
if run_mode == "direct":
    import completion as cmplt
    print(cmplt.main(target=target,path=path))
elif run_mode == "daemon":
    import client
    print(client.main(target=target,path=path))
else:
    try:
        import client
        sys.stderr = open(os.devnull, 'w')
        print(client.main(target=target,path=path))
    except:
        import completion as cmplt
        print(cmplt.main(target=target,path=path))

