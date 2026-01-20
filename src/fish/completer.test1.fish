set -g _polLingua_cmd "rust_test"

# 1. 禁用 Fish 原生的“无协议”文件补全（可选，如果想完全接管）
# set -g fish_complete_path_nonglob 0

# 2. 定义我们的拼音补全生成器
function __polingua_get_candidates
    set -l ctoken (commandline -ct)
        commandline -t ""
        sleep 1
        eval $_polLingua_cmd "$ctoken" "$PWD" | while read -l cand
            # 输出格式：候选项 \t 描述
            #echo $cand
            printf  "$cand\tfoobar\n"
	end
end

#complete -p "*" -f -a "(__polingua_get_candidates)"
complete -c "*" -f -a "(__polingua_get_candidates)"
