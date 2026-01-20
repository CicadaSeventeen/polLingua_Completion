#!/usr/bin/env fish
set _script_path (readlink -f (dirname (status filename)))
set _polLingua_python_path (dirname $_script_path)'/python'
set _polLingua_commands "PYTHONPYCACHEPREFIX=$XDG_RUNTIME_DIR python3 $_polLingua_python_path/main.py auto"
set -e _script_path
if not set -q COMPLETION_CMD_DIR_ONLY
    set -gx COMPLETION_CMD_DIR_ONLY cd pushd
end

function polLingua_complete
    # 获取当前光标下的词
	set -l current_token (commandline -ct)
	set -l all_token (commandline --tokenize)
	if contains $all_token[-2] $COMPLETION_CMD_DIR_ONLY
		set -x COMPLETION_FILE_TYPE dir
	else
		if set -q COMPLETION_CMD_DIR_ONLY
			set -e COMPLETION_FILE_TYPE
		end
	end
	if  test -n (string trim -- "$current_token")
		set -l ret (eval $_polLingua_commands $current_token| fzf)
		commandline -rt $ret
	end
end

# 将其绑定到 Tab 键（注意：这会覆盖默认补全，需谨慎）
bind \t polLingua_complete
