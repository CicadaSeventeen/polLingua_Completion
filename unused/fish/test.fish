#!/usr/bin/env fish
if test (printf "%s\n%s\n" $3.5.0 $FISH_VERSION | sort -V | tail -1) = $FISH_VERSION
    set  _file_path_tmp (status --current-filename)
else
    set _file_path_tmp  (status --filename)
end
set -l _script_path (dirname (readlink -f {$_file_path_tmp}))
set -e  _file_path_tmp
set _polLingua_python_path (dirname $_script_path)'/python'
set  _polLingua_commands "PYTHONPYCACHEPREFIX=$XDG_RUNTIME_DIR python3 $_polLingua_python_path/main.py auto"
[ -z {$COMPLETION_CMD_DIR_ONLY} ] && export COMPLETION_CMD_DIR_ONLY=cd

function __fish_complete_path --description 'Complete using path'
    set -l target
    set -l description
    switch (count $argv)
        case 0
            # pass
        case 1
            set target "$argv[1]"
        case 2 "*"
            set target "$argv[1]"
            set description "$argv[2]"
    end
    if [ -z $target ]
        set target $PWD'/'
    end
    set -l targets (complete -C"'' $target") (eval $_polLingua_commands $target $PWD)
    if set -q targets[1]
        printf "%s\n" $targets\t"$description"
    end
end


