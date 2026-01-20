# 1. 基础配置
set -g _polLingua_script_path (status filename)
set -g _polLingua_python_dir (dirname (dirname $_polLingua_script_path))'/python'
# 预组装命令，减少 eval 开销
set -g _polLingua_cmd "PYTHONPYCACHEPREFIX=$XDG_RUNTIME_DIR" python3 "$_polLingua_python_dir/main.py" auto
# 1. 定义候选项生成逻辑（保持简洁）
function __polingua_get_candidates
    set -l ctoken (commandline -ct)
    test -z "$ctoken"; and return
    string match -q "-*" -- $ctoken; and return

    # 调用 Python 脚本
    eval $_polLingua_cmd -- "$ctoken" "$PWD" | while read -l cand
        printf "%s\tPolingua\n" "$cand"
    end
end

# 2. 核心：劫持所有已知和未来的补全
# 我们使用 complete 命令的 -c (command) 模式，但不指定具体命令
# 而是针对所有已经加载过的补全定义

function __polingua_inject_all --on-event fish_prompt
    # 获取当前命令行中的第一个单词（即命令名）
    set -l cmd (commandline -po)[1]
    test -z "$cmd"; and return

    # 检查这个命令是否已经注入过，避免重复注入导致性能下降
    if not set -q __polingua_injected_$cmd
        # 强制为该具体命令添加拼音补全规则
        # -f: 允许文件补全
        # -u: 告诉 fish 不要对结果进行文件名检查（防止因为拼音不是真实文件而过滤）
        complete -c "$cmd" -f -a "(__polingua_get_candidates)"
        set -g __polingua_injected_$cmd 1
    end
end

complete -p "" -f -a "(__polingua_inject_all)"

