#!/usr/bin/env bash

_POLLINGUA_COMPLETION_CMD_FALLBACK=pollingua-completion-core
_POLLINGUA_COMPLETION_CMD=( ${POLLINGUA_COMPLETION_CMD:-$_POLLINGUA_COMPLETION_CMD_FALLBACK} )
#function polingua/ble/set-up-completion {
#	bleopt complete_auto_delay=300
#}
#ble-import core-complete -C 'polingua/ble/set-up-completion'

function ble/complete/source:file/.construct-pathname-pattern {
	local path=$1 pattern
	case :$comp_type: in
	(*:a:*) ble/complete/source:file/.construct-ambiguous-pathname-pattern "$path"; pattern=$ret ;;
	(*:A:*) ble/complete/source:file/.construct-ambiguous-pathname-pattern "$path" 0; pattern=$ret ;;
	(*:m:*) ble/string#quote-word "$path"; pattern=*$ret* ;;
	(*)  ble/string#quote-word "$path"; pattern=$ret*
	esac

	if [[ -n "$path" ]]; then
		local pol_output
		# TODO: 区分文件和目录，如cd等
		pol_output=$(${_POLLINGUA_COMPLETION_CMD[@]} "$path" "$PWD")

		if [[ -n "$pol_output" ]]; then
			local line quoted_line
			while read -r line; do
				[[ -z $line ]] && continue
				ble/string#quote-word "$line"
				quoted_line=$ret
				pattern="$pattern $quoted_line"
			done <<< "$pol_output"
		fi
	fi
	ret=$pattern
}
