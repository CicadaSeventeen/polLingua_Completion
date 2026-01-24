#!/usr/bin/env zsh
autoload -Uz compinit
zmodload zsh/parameter
setopt extended_glob
compinit
zstyle ':completion:*' insert-tab false
zstyle ':pollingua-completion:settings' enable-internal-completers  _list
zstyle ':completion:*' completer _polingua _match _history _approximate _list
bindkey "^I" menu-complete
