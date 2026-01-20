set _source_path (readlink -f (dirname (status --current-filename)))
source $_source_path/fish/daemon.fish
source $_source_path/fish/completer.fish
set -e _source_path
