_polLingua_python_path=$(realpath ${${(%):-%x}:h})'/python'
autoload -Uz zsh/parameter
declare -A _polLingua_commands
_polLingua_commands[startswith]="python3 ${_polLingua_python_path}/main_startswith.py"
_polLingua_commands[equal]="python3 ${_polLingua_python_path}/main_equal.py"

_polLingua_core(){
    local _last_word=$(
    (
        IFS=$'\ \t\n\0'
        eval "set -- $words[CURRENT]"
        printf %q ${argv[-1]}
    ))
    compstate[pattern_match]='*'
    if [[ $_last_word == '' ]];then
        shift
        _files $@
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
        corrections=( ${(f)${(s:\n:)"$(eval $_polLingua_commands[$1] $_last_word $PWD)"}})
        shift
        #echo $_tmp_dirname $_word
        for _word in ${corrections}
        do
            #local _tmp_showname=$(printf %q $_tmp_dirname$_word)
            if [[ -d $(eval realpath $_tmp_dirname$_word) ]];then
                if [[ $_tmp_dirname == '' ]];then
                    compadd -D 'Directories' -f -U -S '/' $@ $_word $_tmp_dirname
                else
                    compadd -D 'Directories' -f -U -S '/' $@ -p $_tmp_dirname $_word
                fi
            else
                if [[ $_tmp_dirname == '' ]];then
                    compadd -D 'Files' -f -U -S '' $@ $_word $_tmp_dirname
                else
                    compadd -D 'Files' -f -U -S '' $@ -p $_tmp_dirname $_word
                fi
            fi
        done
    fi
    return 1
}

_polLingua_startswith() {
    [[ _matcher_num -gt 2 ]] && return 1
    _polLingua_core startswith $@
}

_polLingua_equal() {
    [[ _matcher_num -gt 2 ]] && return 1
    _polLingua_core equal $@
}

