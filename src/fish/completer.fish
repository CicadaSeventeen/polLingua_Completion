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
	if set -q _polingua_last_candidate; and set -q _polingua_candidates_list; and [ "$current_token" = "$_polingua_last_candidate" ]
		# 继续上次补全
		set -l candidates_list_len (count $_polingua_candidates_list)
		set -l candidate_count (contains -i "$_polingua_last_candidate" $_polingua_candidates_list)
		echo "$_polingua_last_candidate" $_polingua_candidates_list > ./txt
		if  test "$candidate_count" -lt "$candidates_list_len"
			# 切换到下一个候选项
			set -l candidate_count (math $candidate_count + 1)
			set -l candidate_word $_polingua_candidates_list[$candidate_count]
			set -g _polingua_last_candidate $candidate_word
			commandline -rt $candidate_word
			tput cud1
			echo $_polingua_candidates_list ...
			tput cuu 2
			commandline -f repaint
			return
		else
			# 进入fish原生补全
				commandline  -rt $_polingua_last_input_token
				set -e _polingua_candidates_list
				set -e _polingua_last_candidate
				set -e _polingua_last_input_token
				commandline  -f complete
				return
		end
	else
		# 进行新的补全
		if  test -n (string trim -- "$current_token")
		set -g _polingua_candidates_list (eval $_polLingua_commands $current_token $PWD)
			if test -n (string trim -- "$_polingua_candidates_list")
				set -g _polingua_last_input_token $current_token
				set -l candidates_num (count $_polingua_candidates_list)
				set -l candidate_word $_polingua_candidates_list[1]
				set -g _polingua_last_candidate $candidate_word
				commandline -rt $candidate_word
				tput cud1
				echo $_polingua_candidates_list ...
				tput cuu 2
				commandline -f repaint
				return
			end
		end
		# 进入fish原生补全
		set -e _polingua_candidates_list
		commandline  -f complete
		return
	end
end

# 将其绑定到 Tab 键（注意：这会覆盖默认补全，需谨慎）
bind \t polLingua_complete
