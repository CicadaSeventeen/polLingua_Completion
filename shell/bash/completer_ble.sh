#!/usr/bin/env bash

# --- 1. 配置桥接 ---
_POLLINGUA_COMPLETION_CMD_FALLBACK=pollingua-completion-core
_POLLINGUA_COMPLETION_CMD=( ${POLLINGUA_COMPLETION_CMD:-$_POLLINGUA_COMPLETION_CMD_FALLBACK} )

# --- 2. 设置 ble.sh 选项 ---
#function polingua/ble/set-up-completion {
## 延迟 300ms 触发自动补全，防止 Python 脚本调用过于频繁导致卡顿
#	bleopt complete_auto_delay=300
#}
#ble-import core-complete -C 'polingua/ble/set-up-completion'

# --- 3. 核心 Monkey Patch ---
# 覆盖 ble.sh 内部函数：将拼音/罗马字候选项注入到路径模式中
function ble/complete/source:file/.construct-pathname-pattern {
	local path=$1 pattern

	# [Part A] 保留 ble.sh 原生逻辑
	# 根据当前的匹配类型 (ambiguous, substring, prefix) 生成基础 Glob 模式
	case :$comp_type: in
	(*:a:*) ble/complete/source:file/.construct-ambiguous-pathname-pattern "$path"; pattern=$ret ;;
	(*:A:*) ble/complete/source:file/.construct-ambiguous-pathname-pattern "$path" 0; pattern=$ret ;;
	(*:m:*) ble/string#quote-word "$path"; pattern=*$ret* ;;
	(*)  ble/string#quote-word "$path"; pattern=$ret*
	esac

	# [Part B] 注入 Polingua (Python) 候选项
	# 只有当 path 不为空时才调用 Python，避免无意义的开销
	if [[ -n "$path" ]]; then
		local pol_output
		# 调用 Python 脚本，传入当前输入和当前路径
		# 注意：这里假设 Python 脚本输出的是以换行符分隔的文件路径列表
		# TODO: 区分文件和目录，如cd等
		pol_output=$(${_POLLINGUA_COMPLETION_CMD[@]} "$path" "$PWD")

		if [[ -n "$pol_output" ]]; then
			local line quoted_line
			# 逐行读取 Python 输出
			while read -r line; do
				[[ -z $line ]] && continue

				# 关键步骤：使用 ble.sh 内置工具对文件名进行安全转义 (加引号)
				# 例如: "测试 文件" -> "'测试 文件'"
				ble/string#quote-word "$line"
				quoted_line=$ret

				# 将转义后的精确路径追加到 pattern 中，用空格分隔
				pattern="$pattern $quoted_line"
			done <<< "$pol_output"
		fi
	fi

	# 返回合并后的模式字符串
	# 下游的 eval 将会执行类似: ( 'inp'* 'input/测试' 'input/其他' )
	ret=$pattern
}
