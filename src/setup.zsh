autoload -Uz compinit
zmodload zsh/parameter
setopt extended_glob
compinit
local _source_path=$(readlink -f ${${(%):-%x}:h})
source $_source_path/zsh/daemon.zsh
source $_source_path/zsh/completer.zsh
zstyle ':completion:*' insert-tab false
zstyle ':completion:*' completer _polLingua_smart
unset _source_path
