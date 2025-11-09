_script_path=$(dirname ${BASH_SOURCE[0]})
_polLingua_python_path=$(dirname $_script_path)'/python'
if [[ `uname` == 'Linux'* ]];then
    python3 $_polLingua_python_path/daemon.py &!;_pid_polLingua_daemon=$!
else
    python3 $_polLingua_python_path/python/daemon.py &!;_pid_polLingua_daemon=$!
fi
[[ `uname` == 'Linux'* ]] ||  trap "kill $_pid_polLingua_daemon;wait $_pid_polLingua_daemon" EXIT
