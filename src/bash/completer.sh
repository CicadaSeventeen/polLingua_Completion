#!/usr/bin/env bash

# ==============================================================================
# 配置区域
# ==============================================================================
# 获取当前脚本所在目录 (兼容 symlink)
#
#_get_script_dir() {
#    local source="${BASH_SOURCE[0]}"
#    while [ -h "$source" ]; do
#        local dir="$( cd -P "$( dirname "$source" )" >/dev/null 2>&1 && pwd )"
#        source="$(readlink "$source")"
#        [[ $source != /* ]] && source="$dir/$source"
#    done
#    cd -P "$( dirname "$source" )" >/dev/null 2>&1 && pwd
#}

#_SCRIPT_PATH=$(_get_script_dir)
# 假设你的命令就在 path 中，或者你需要指定绝对路径
# 建议使用数组来存储命令和参数，比字符串拼接更安全
_POLLINGUA_COMPLETION_CMD_FALLBACK=rust_test
_POLLINGUA_COMPLETION_CMD=( ${POLLINGUA_COMPLETION_CMD:-$_POLLINGUA_COMPLETION_CMD_FALLBACK} )
#export COMPLETION_FILENAME_MATCH_MODE=startswith

# ==============================================================================
# Hook 核心逻辑
# ==============================================================================

# 检查是否已安装 bash-completion
if ! declare -F _comp_compgen_filedir &>/dev/null; then
    # 尝试加载系统的 bash_completion（可选，视环境而定）
    if [ -f /usr/share/bash-completion/bash_completion ]; then
        . /usr/share/bash-completion/bash_completion
    fi

    if ! declare -F _comp_compgen_filedir &>/dev/null; then
        echo "[PolLingua] Error: _comp_compgen_filedir not found. Ensure bash-completion is loaded." >&2
        return 1
    fi
fi

# 防止重复 Hook (幂等性检查)
if [[ -n "${_POLLINGUA_HOOKED+x}" ]]; then
    return 0
fi
export _POLLINGUA_HOOKED=1

# 定义 Hook 函数生成器
# 参数 1: 原函数名
# 参数 2: 备份函数名
_inject_hook() {
    local target_func="$1"
    local backup_func="$2"

    # 如果备份函数不存在，才进行备份，避免多次 source 导致无限递归
    if ! declare -F "$backup_func" &>/dev/null; then
        # 获取原函数代码，去掉第一行(函数定义)，保留函数体
        local func_body
        func_body=$(declare -f "$target_func" | tail -n +2)

        # 定义备份函数
        eval "function ${backup_func}() { $func_body }"

        # 重写原函数
        # 注意：这里我们使用 source 时的动态定义，而不是直接写死在文件里
        eval "${target_func}() {
            ${backup_func} \"\$@\"
            _pinyin_completion \"\$@\"
        }"
    fi
}

# 执行 Hook
_inject_hook "_comp_compgen_filedir"       "__bak_comp_compgen_filedir"
_inject_hook "_comp_compgen_filedir_xspec" "__bak_comp_compgen_filedir_xspec"
_inject_hook "_comp_complete_minimal"      "__bak_comp_complete_minimal"
_inject_hook "_comp_expand_glob"           "__bak_comp_expand_glob"

# ==============================================================================
# 拼音补全逻辑
# ==============================================================================

_pinyin_completion() {
    # 基础检查：是否有补全单词
    if [ ${#COMP_WORDS[@]} -eq 0 ] || [ -z "${COMP_CWORD+x}" ]; then
        return
    fi

    local cur="${COMP_WORDS[COMP_CWORD]}"

    # 忽略空字符串，避免触发全量搜索
    [ -z "$cur" ] && return

    # 检测 "~/" 开头
    local home_start=false
    if [[ "${cur:0:2}" == "~/" ]]; then
        home_start=true
    fi

    # 准备调用外部命令的参数
    #local cmd_mode="file"
    local cmd_mode="all"
    #local compgen_flag="-f"
    # 检测是否只补全目录 (-d 参数)
    # 注意：这里检测的是传入 _pinyin_completion 的第一个参数，通常是 _comp_compgen_filedir 传进来的
    if [[ "${1-}" == -d ]]; then
        cmd_mode="dir"
       # compgen_flag="-d"
    fi

    # 获取外部命令输出
    local -a pinyin_matched=()

    # !!! 安全修复：移除 eval，直接执行命令数组 !!!
    # 使用 env 传递变量比 eval 更安全
    if output=$(_POLINGUA_COMPLETION_FILETYPE="$cmd_mode" "${_POLLINGUA_COMPLETION_CMD[@]}" "$cur" "$PWD" 2>/dev/null); then
        # 兼容性处理：readarray (mapfile) 在 Bash 4.0+ 可用
        if [ -n "$output" ]; then
            readarray -t pinyin_matched <<< "$output"
        fi
    fi

    # 如果没有匹配结果，直接返回
    if [ ${#pinyin_matched[@]} -eq 0 ]; then
        return
    fi

    # 启用文件名处理选项（处理空格等转义）
    compopt -o filenames 2>/dev/null

    # ==========================================================================
    # 结果合并与去重 (修复 "数组下标不正确" 的核心部分)
    # ==========================================================================

    # 将现有的 COMPREPLY 和新的匹配项合并
    local -a combined=("${COMPREPLY[@]}" "${pinyin_matched[@]}")

    # 清空 COMPREPLY 准备重写
    COMPREPLY=()

    declare -A seen
    local item

    for item in "${combined[@]}"; do
        # 1. 必须非空检查
        [[ -z "$item" ]] && continue

        # 2. 检查关联数组中是否已存在 (使用 +exists 技巧)
        if [[ -z "${seen["$item"]+exists}" ]]; then
            seen["$item"]=1
            COMPREPLY+=("$item")
        fi
    done
    unset seen

    # ==========================================================================
    # 处理波浪号 (~) 路径
    # ==========================================================================
    if [[ "$home_start" == true ]]; then
        local i
        # 预先计算长度，避免循环中重复计算
        local home_len=${#HOME}

        for i in "${!COMPREPLY[@]}"; do
            local val="${COMPREPLY[$i]}"
            # 检查是否以 $HOME 开头
            if [[ "$val" == "$HOME"/* ]]; then
                # 替换 $HOME 为 ~
                COMPREPLY[$i]="~${val:$home_len}"
            fi
        done
    fi
}
