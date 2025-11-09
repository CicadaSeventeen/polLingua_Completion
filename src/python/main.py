import sys
import os
def solve_arg():
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
    return [run_mode,target,path]

def get_string(run_mode,target,path):
    if run_mode == "direct":
        import completion as cmplt
        return_string = cmplt.main(target=target,path=path)
    elif run_mode == "daemon":
        import client
        return_string = client.main(target=target,path=path)
    else:
        try:
            import client
            sys.stderr = open(os.devnull, 'w')
            return_string = client.main(target=target,path=path)
        except:
            import completion as cmplt
            return_string = cmplt.main(target=target,path=path)
    return return_string

def main():
    [run_mode,target,path] = solve_arg()
    return_string = get_string(run_mode=run_mode,target=target,path=path)
    print(return_string)

if __name__ == "__main__":
    main()
