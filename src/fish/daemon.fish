#!/usr/bin/env fish
set _script_path (readlink -f (dirname (status --current-filename)))
set _polLingua_python_path (dirname $_script_path)/python

if string match -q 'Linux*' (uname)
	python3 $_polLingua_python_path/daemon.py &
	set _pid_polLingua_daemon $last_pid
else
	python3 $_polLingua_python_path/python/daemon.py &
	set _pid_polLingua_daemon $last_pid
end

function __cleanup_polLingua --on-event fish_exit
	sleep 0.2
	if ps -p $_pid_polLingua_daemon
		kill $_pid_polLingua_daemon
	end
	sleep 0.2
	if ps -p $_pid_polLingua_daemon
		kill -9 $_pid_polLingua_daemon
	end
end
