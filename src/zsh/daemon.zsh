autoload -Uz zsh/parameter
setopt extended_glob
local _script_path=$(readlink -f ${${(%):-%x}:h})
local _polLingua_python_path=$(dirname $_script_path)'/python'
if [[ `uname` == (#i)'Linux'* ]];then
    python3 $_polLingua_python_path/daemon.py &!;_pid_polLingua_daemon=$!
else
    python3 $_polLingua_python_path/python/daemon.py &!;_pid_polLingua_daemon=$!
fi
[[ `uname` == (#i)'Linux'* ]] ||  trap "kill $_pid_polLingua_daemon;wait $_pid_polLingua_daemon" EXIT
