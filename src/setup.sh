_source_path=$(readlink -f $(dirname ${BASH_SOURCE[0]}))
source $_source_path/bash/daemon.sh
source $_source_path/bash/completer.sh
unset _source_path
