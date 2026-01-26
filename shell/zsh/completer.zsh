#!/usr/bin/env zsh
[ -z ${POLLINGUA_COMPLETION_CMD_DIR_ONLY} ] && POLLINGUA_COMPLETION_CMD_DIR_ONLY=(cd pushd  rmdir chroot)
local _polLingua_cmd=(pollingua-completion-core)
zstyle -e ':pollingua-completion:settings'  enable-internal-completers '[[ -z $reply ]] && reply=(_list)'
#zstyle -e ':pollingua-completion:settings'  enable-fzf '[[ -z $reply ]] && reply=false'

_polingua() {
    [[ _matcher_num -gt 1 ]] && return 1

	local ret=1
	local -a expl
	local -a matches

	local cur_word="$words[CURRENT]"
	local last_word="$words[CURRENT-1]"
	cmd_mode=all
	if [[ ${POLLINGUA_COMPLETION_CMD_DIR_ONLY[(r)$last_word]} == $last_word ]]
	then
		cmd_mode=dir
	fi

	zstyle -a ':pollingua-completion:settings' enable-internal-completers enable_internal_completers
	#zstyle -a ':pollingua-completion:settings' enable-fzf enable_fzf
	if [[ -d "$expand_dir" && -n "$base_name" ]]; then
		_pollingua_ret=($(_POLINGUA_COMPLETION_FILETYPE=$cmd_mode ${_polLingua_cmd} "${cur_word}" "$PWD"))
		compadd -f -U -Q -a _pollingua_ret
		for item in $enable_internal_completers
		do
			$item
		done
	fi
	return ret
}
