autoload -Uz zsh/parameter
setopt extended_glob
local _script_path=$(readlink -f ${${(%):-%x}:h})
local _polLingua_python_path=$(dirname $_script_path)'/python'
if [[ `uname` == (#i)'Linux'* ]];then
    python3 $_polLingua_python_path/daemon.py &!;_pid_polLingua_daemon=$!
else
    python3 $_polLingua_python_path/python/daemon.py &!;_pid_polLingua_daemon=$!
fi

_cleanup() {
	sleep 0.2
	( ps -p $_pid_polLingua_daemon ) && {
		kill $_pid_polLingua_daemon
	}
	sleep 0.2
	( ps -p $_pid_polLingua_daemon ) && {
		kill -9  $_pid_polLingua_daemon
	}
}

trap _cleanup EXIT
