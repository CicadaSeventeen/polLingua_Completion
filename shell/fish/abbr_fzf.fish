#!/usr/bin/env fish
function __pollingua_abbr_double_colon
	string match -rg '^::([^/]+)::$' -- $argv[1] | read -lat name
	or return
	pollingua-completion-core "$name" "$PWD" | fzf
end

function __pollingua_abbr_at
	string match -rg '^@([^/]+)@$' -- $argv[1] | read -lat name
	or return
	pollingua-completion-core "$name" "$PWD" | fzf
end

function __pollingua_abbr_everywhere
	set -l name $argv[1]
	# TODO: 区分文件和目录，如cd等
	#set -l all_token (commandline --tokenize)
	#echo $all_token[-2]
	set -l output (pollingua-completion-core "$name" "$PWD"  |  string trim)
	if test -z $output[1]
		return 1
	else
		printf "%s\n" $output | fzf
	end
end

abbr pollingua_abbr_double_colon --position anywhere --regex '::.*::' --function __pollingua_abbr_double_colon
abbr pollingua_abbr_at --position anywhere --regex '@.*@' --function __pollingua_abbr_at
abbr pollingua_abbr_everywhere --position anywhere --regex '.*'  --function __pollingua_abbr_everywhere
