autoload -Uz compinit
zmodload zsh/parameter
setopt extended_glob
compinit
zstyle ':completion:*' insert-tab false
zstyle ':completion:*' completer _polLingua_smart
if [[ `uname` == (#i)'Linux'* ]];then
    python3 ./python/daemon.py &!;_pid_polLingua_daemon=$!
else
    python3 ./python/daemon.py &!;_pid_polLingua_daemon=$!
fi
[[ `uname` == (#i)'Linux'* ]] ||  trap "kill $_pid_polLingua_daemon;wait $_pid_polLingua_daemon" EXIT
