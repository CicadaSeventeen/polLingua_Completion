#!/usr/bin/env fish
function __pollingua_abbr_double_colon
	string match -rg '^::([^/]+)::$' -- $argv[1] | read -lat name
	or return
	pollingua-completion-core "$name" "$PWD"
end

function __pollingua_abbr_at
	string match -rg '^@([^/]+)@$' -- $argv[1] | read -lat name
	or return
	pollingua-completion-core "$name" "$PWD"
end

function __pollingua_abbr_everywhere
	set -l name $argv[1]
	# TODO: 区分文件和目录，如cd等
	#set -l all_token (commandline --tokenize)
	set -l output (pollingua-completion-core "$argv[1]" "$PWD" | string trim)
	if test -z $output[1]
		return 1
	else
		echo $output[1]
	end
end

abbr pollingua_abbr_double_colon --position anywhere --regex '::.*::' --function __pollingua_abbr_double_colon
abbr pollingua_abbr_at --position anywhere --regex '@.*@' --function __pollingua_abbr_at
#abbr pollingua_abbr_everywhere --position anywhere --regex '.*'  --function __pollingua_abbr_everywhere
