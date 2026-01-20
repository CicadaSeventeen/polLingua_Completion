#autoload

# 这是一个 Completer，应该在 .zshrc 中配置:
# zstyle ':completion:*' completers _polingua _complete _ignored

#local _polLingua_script_path=${${(%):-%x}:h}
# 假设 python 脚本在 ../python/main.py
local _polLingua_cmd=(rust_test)
zstyle -e ':pollingua-completion:settings'  enable-internal-completers '[[ -z $reply ]] && reply=(_list)'

_polingua() {
    [[ _matcher_num -gt 1 ]] && return 1

	local ret=1
	local -a expl
	local -a matches
	# 3. 核心魔法：处理路径前缀
	# 如果当前词包含路径（如 foo/ba），_path_files 通常会处理它。
	# 这里我们手动处理一部分，为了传给 Python 正确的上下文。

	local cur_word="$words[CURRENT]"
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

	# 4. 调用 Python 获取候选项
	# 你的 Python 脚本应该接收: "当前正在输入的词(base_name)" "所在的目录(expand_dir)"
	# 并返回：该目录下的文件名（不带路径前缀！）
	# 优化：仅当目录存在时才调用
	zstyle -a ':pollingua-completion:settings' enable-internal-completers enable_internal_completers
	if [[ -d "$expand_dir" && -n "$base_name" ]]; then
		compadd -f -U $(eval ${_polLingua_cmd} "${base_name}" "${expand_dir}")
		for item in $enable_internal_completers
		do
			eval $item
		done
	fi
	return ret
}
