_script_path=$(readlink -f $(dirname ${BASH_SOURCE[0]}))
_polLingua_python_path=$(dirname $_script_path)'/python'
_polLingua_commands="PYTHONPYCACHEPREFIX=$XDG_RUNTIME_DIR python3 ${_polLingua_python_path}/main.py auto"

function polingua/ble/set-up-completion {
  bleopt complete_auto_delay=300
}
ble-import core-complete -C 'polingua/ble/set-up-completion'

ble/complete/source:file ()
{
	local opts=$1;
	[[ $comps_flags == *v* ]] || return 1;
	[[ :$comp_type: != *:[maA]:* && $COMPV =~ ^.+/ ]] && COMP_PREFIX=${BASH_REMATCH[0]};
	[[ :$comp_type: == *:[maA]:* && ! -n $COMPV ]] && return 1;
	ble/complete/source:tilde;
	local ext=$?;
	((ext==148||ext==0)) && return "$ext";
	local -a candidates=();
	local ret;
	local tm
#	local -a tmp_array;
	local list_of_polingua;
#	function _polLingua_if_exist(){
#		local element=$1
#		shift
#		for item in "$@"
#		do
#			if [[ "$item" == "$element" ]]
#			then
#				return 0
#			fi
#		done
#		return 1
#	}
	ble/complete/source:file/.construct-pathname-pattern "$COMPV";
	tmp_ret=$ret;
	[[ :$opts: == *:directory:* ]] && ret=$ret/;
	list_of_polingua=$(eval $_polLingua_commands "$COMPV" $PWD);
	for term_of_polingua in "$list_of_polingua"
	do
		ble/complete/source:file/.construct-pathname-pattern "$term_of_polingua";
		[[ :$opts: == *:directory:* ]] && ret=$ret/;
		tmp_ret="$tmp_ret"' '"$ret"
	done
	ret=$(echo "$tmp_ret"|awk '{for(i=1;i<=NF;i++)if(!seen[$i]++)print($i)}')
#	unset _polLingua_if_exist
	ble/complete/util/eval-pathname-expansion "$ret";
	(($?==148)) && return 148;
	ble/complete/source/test-limit "${#ret[@]}" || return 1;
	if [[ :$opts: == *:directory:* ]]; then
		candidates=("${ret[@]%/}");
	else
		candidates=("${ret[@]}");
	fi;
	[[ :$opts: == *:no-fd:* ]] && ble/array#remove-by-regex candidates '^[0-9]+-?$|^-$';
	[[ :$opts: == *:filter-by-regex:* ]] && ble/array#filter-by-regex candidates "$source_file_regex";
	[[ :$opts: == *:filter:* ]] && ble/array#filter candidates "$source_file_filter";
	local action=file ret=;
	ble/opts#extract-last-optarg "$opts" action;
	[[ -n $ret ]] && action=$ret;
	local flag_source_filter=1;
	ble/complete/cand/yield-filenames "$action" "${candidates[@]}"
}
