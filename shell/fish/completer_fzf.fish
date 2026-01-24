#!/usr/bin/env fish
if not set -q _POLLINGUA_COMPLETION_CMD_DIR_ONLY
	set -gx _POLLINGUA_COMPLETION_CMD_DIR_ONLY cd pushd rmdir chroot
end

function __polLingua_complete_fzf
    # 获取当前光标下的词
	set -l current_token (commandline -ct)
	if test -n $current_token
		set -l output (pollingua-completion-core "$current_token" "$PWD"  |  string trim)
		if test -z $output[1]
			true
		else
			set -l ret (printf "%s\n" $output | fzf)
			commandline -rt $ret
		end
	end
	commandline -f complete
end

# 将其绑定到 Tab 键（注意：这会覆盖默认补全）
bind \t __polLingua_complete_fzf
