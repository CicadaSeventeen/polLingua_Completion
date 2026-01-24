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
	# 3. 核心魔法：处理路径前缀
	# 如果当前词包含路径（如 foo/ba），_path_files 通常会处理它。
	# 这里我们手动处理一部分，为了传给 Python 正确的上下文。

	local cur_word="$words[CURRENT]"
	local last_word="$words[CURRENT-1]"
	cmd_mode=all
	if [[ ${POLLINGUA_COMPLETION_CMD_DIR_ONLY[(r)$last_word]} == $last_word ]]
	then
		cmd_mode=dir
	fi
	local dir_prefix="${cur_word:h}"
	local base_name="${cur_word:t}"

	# 如果没有路径分隔符，dirname 是 . (或者当前词就是 .)
	if [[ "$cur_word" != */* ]]; then
		dir_prefix="."
		base_name="$cur_word"
	elif [[ "$cur_word" == */ ]]; then
		# 输入是 "src/" 的情况
		dir_prefix="${cur_word%/}"
		base_name=""
	fi

	# 处理 ~user 或 ~/ 扩展
	local expand_dir=$dir_prefix
	if [[ "$dir_prefix" == \~* ]]; then
		eval "expand_dir=$dir_prefix"
	fi

	# 你的 Python 脚本应该接收: "当前正在输入的词(base_name)" "所在的目录(expand_dir)"
	# 并返回：该目录下的文件名（不带路径前缀！）
	zstyle -a ':pollingua-completion:settings' enable-internal-completers enable_internal_completers
	#zstyle -a ':pollingua-completion:settings' enable-fzf enable_fzf
	if [[ -d "$expand_dir" && -n "$base_name" ]]; then
		_pollingua_ret=($(_POLINGUA_COMPLETION_FILETYPE=$cmd_mode ${_polLingua_cmd} "${base_name}" "${expand_dir}"))
		compadd -f -U -Q -a _pollingua_ret
		for item in $enable_internal_completers
		do
			$item
		done
	fi
	return ret
}
