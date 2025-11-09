zmodload zsh/parameter
setopt extended_glob
local _script_path=$(readlink -f ${${(%):-%x}:h})
local _polLingua_python_path=$(dirname $_script_path)'/python'
local _polLingua_commands="PYTHONPYCACHEPREFIX=$XDG_RUNTIME_DIR python3 ${_polLingua_python_path}/main.py auto"
local _polLingua_turbo_commands="PYTHONPYCACHEPREFIX=$XDG_RUNTIME_DIR python3 ${_polLingua_python_path}/zsh_compadd.py auto"
[ -z ${COMPLETION_CMD_DIR_ONLY} ] && { export COMPLETION_CMD_DIR_ONLY=cd; }
#COMPLETION_CMD_FILE_ONLY=""


_polLingua_startswith() {
    [[ _matcher_num -gt 2 ]] && return 1
    COMPLETION_FILENAME_MATCH_MODE=startswith _polLingua_core $@
}

_polLingua_equal() {
    [[ _matcher_num -gt 2 ]] && return 1
    COMPLETION_FILENAME_MATCH_MODE=equal _polLingua_core $@
}

_polLingua_file_startswith() {
    export COMPLETION_FILE_TYPE=file
    [[ _matcher_num -gt 2 ]] && return 1
    COMPLETION_FILENAME_MATCH_MODE=startswith _polLingua_core $@
    unset COMPLETION_FILE_TYPE
}

_polLingua_file_equal() {
    [[ _matcher_num -gt 2 ]] && return 1
    export COMPLETION_FILE_TYPE=file
    COMPLETION_FILENAME_MATCH_MODE=equal _polLingua_core $@
    unset COMPLETION_FILE_TYPE
}

_polLingua_dir_startswith() {
    [[ _matcher_num -gt 2 ]] && return 1
    export COMPLETION_FILE_TYPE=dir
    COMPLETION_FILENAME_MATCH_MODE=startswith _polLingua_core $@
    unset COMPLETION_FILE_TYPE
}

_polLingua_dir_equal() {
    [[ _matcher_num -gt 2 ]] && return 1
    export COMPLETION_FILE_TYPE=dir
    COMPLETION_FILENAME_MATCH_MODE=equal _polLingua_core $@
    unset COMPLETION_FILE_TYPE
}

_polLingua_smart(){
    [[ _matcher_num -gt 2 ]] && return 1
    if [[ -n $words[1] && ":$COMPLETION_CMD_DIR_ONLY:" == *":$words[1]:"* ]]; then
            _polLingua_dir_startswith $@
    elif [[ -n $words[1] && ":$COMPLETION_CMD_FILE_ONLY:" == *":$words[1]:"* ]];then
            _polLingua_file_startswith $@
    else
        _polLingua_startswith $@
    fi
}

_polLingua_core_legacy(){
    local _last_word=$(
    (
        IFS=$'\ \t\n\0'
        eval "set -- $words[CURRENT]"
        printf %q ${argv[-1]}
    ))
    compstate[pattern_match]='*'
    if [[ $_last_word == '' ]];then
        if [[ $COMPLETION_FILE_TYPE == 'file' ]];then
            _files -g '*(-.)' $@
        elif [[ $COMPLETION_FILE_TYPE == 'dir' ]];then
            _files -/ $@
        else
            _files $@
        fi
    else
        if [[ $_last_word == */ ]];then
            local _tmp_dirname=${_last_word%/}
        else
            local _tmp_dirname=$(dirname $_last_word)
            if [[ $words[CURRENT] == '~'* && $_tmp_dirname == "$HOME"* ]];then
                _tmp_dirname='~'${_tmp_dirname#$HOME}
            fi
        fi
        if [[ $_tmp_dirname == '.' ]];then
            if [[ $_last_word == './'* ]];then
                _tmp_dirname='./'
            else
                _tmp_dirname=''
            fi
        elif [[ $_tmp_dirname == '/' ]];then
            :
        else
            _tmp_dirname=$_tmp_dirname'/'
        fi
        local -a corrections
        corrections=( ${(f)${(s:\n:)"$(eval $_polLingua_commands $_last_word $PWD)"}})
        #compadd -f -U -S '/' $corrections[@]
#:<<EOF
        for _word in ${corrections}
        do
            #local _tmp_showname=$(printf %q $_tmp_dirname$_word)
           if [[ -d $(eval realpath $_tmp_dirname$_word) ]];then
                if [[ $_tmp_dirname == '' ]];then
                    compadd -f -U -S '/' -Q $@ $_word $_tmp_dirname
                else
                    compadd -f -U -S '/' -Q $@ -p $_tmp_dirname $_word
                fi
            else
                if [[ $_tmp_dirname == '' ]];then
                    compadd -f -U -S '' -Q $@ $_word $_tmp_dirname
                else
                    compadd -f -U -S '' -Q $@ -p $_tmp_dirname $_word
                fi
            fi
        done
#EOF
    fi
    return 1
}

_polLingua_core_turbo(){
    [[ _matcher_num -gt 2 ]] && return 1
    local _last_word=$(
    (
        IFS=$'\ \t\n\0'
        eval "set -- $words[CURRENT]"
        printf %q ${argv[-1]}
    ))
    compstate[pattern_match]='*'
    if [[ $_last_word == '' ]];then
        if [[ $COMPLETION_FILE_TYPE == 'file' ]];then
            _files -g '*(-.)' $@
        elif [[ $COMPLETION_FILE_TYPE == 'dir' ]];then
            _files -/ $@
        else
            _files $@
        fi
    else
        if [[ $_last_word == */ ]];then
            local _tmp_dirname=${_last_word%/}
        else
            local _tmp_dirname=$(dirname $_last_word)
            if [[ $words[CURRENT] == '~'* && $_tmp_dirname == "$HOME"* ]];then
                _tmp_dirname='~'${_tmp_dirname#$HOME}
            fi
        fi
        if [[ $_tmp_dirname == '.' ]];then
            if [[ $_last_word == './'* ]];then
                _tmp_dirname='./'
            else
                _tmp_dirname=''
            fi
        elif [[ $_tmp_dirname == '/' ]];then
            :
        else
            _tmp_dirname=$_tmp_dirname'/'
        fi
        #echo $_polLingua_turbo_commands $_last_word $PWD
        cmdline=$(eval $_polLingua_turbo_commands $_last_word $PWD)
        #echo $cmdline
        eval "$cmdline"
    fi
    return 1
}

_polLingua_core(){
    _polLingua_core_turbo $@
    return $?
}
