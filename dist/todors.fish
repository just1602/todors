# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_todors_global_optspecs
	string join \n c/config= h/help V/version
end

function __fish_todors_needs_command
	# Figure out if the current invocation already has a command.
	set -l cmd (commandline -opc)
	set -e cmd[1]
	argparse -s (__fish_todors_global_optspecs) -- $cmd 2>/dev/null
	or return
	if set -q argv[1]
		# Also print the command, so this can be used to figure out what it is.
		echo $argv[1]
		return 1
	end
	return 0
end

function __fish_todors_using_subcommand
	set -l cmd (__fish_todors_needs_command)
	test -z "$cmd"
	and return 1
	contains -- $cmd[1] $argv
end

complete -c todors -n "__fish_todors_needs_command" -s c -l config -d 'Path to the config file.' -r -F
complete -c todors -n "__fish_todors_needs_command" -s h -l help -d 'Print help'
complete -c todors -n "__fish_todors_needs_command" -s V -l version -d 'Print version'
complete -c todors -n "__fish_todors_needs_command" -f -a "add" -d 'Add a task to the list'
complete -c todors -n "__fish_todors_needs_command" -f -a "a" -d 'Add a task to the list'
complete -c todors -n "__fish_todors_needs_command" -f -a "done" -d 'Mark selected tasks as done'
complete -c todors -n "__fish_todors_needs_command" -f -a "do" -d 'Mark selected tasks as done'
complete -c todors -n "__fish_todors_needs_command" -f -a "list" -d 'List all the tasks or those that match the query'
complete -c todors -n "__fish_todors_needs_command" -f -a "ls" -d 'List all the tasks or those that match the query'
complete -c todors -n "__fish_todors_needs_command" -f -a "remove" -d 'Remove selected item from the todo file'
complete -c todors -n "__fish_todors_needs_command" -f -a "rm" -d 'Remove selected item from the todo file'
complete -c todors -n "__fish_todors_needs_command" -f -a "edit" -d 'Edit the todo file with a text editor'
complete -c todors -n "__fish_todors_needs_command" -f -a "due" -d 'List all due tasks'
complete -c todors -n "__fish_todors_needs_command" -f -a "undone" -d 'Mark selected tasks as not done'
complete -c todors -n "__fish_todors_needs_command" -f -a "undo" -d 'Mark selected tasks as not done'
complete -c todors -n "__fish_todors_needs_command" -f -a "clean" -d 'Clean all the completed tasks'
complete -c todors -n "__fish_todors_needs_command" -f -a "modify" -d 'Modify selected tasks as desired'
complete -c todors -n "__fish_todors_needs_command" -f -a "mod" -d 'Modify selected tasks as desired'
complete -c todors -n "__fish_todors_needs_command" -f -a "next" -d 'Show the next task to do base on the urgency task sort we have'
complete -c todors -n "__fish_todors_needs_command" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c todors -n "__fish_todors_using_subcommand add" -l pri -d 'Set the priority directly after creating the task' -r
complete -c todors -n "__fish_todors_using_subcommand add" -s h -l help -d 'Print help'
complete -c todors -n "__fish_todors_using_subcommand a" -l pri -d 'Set the priority directly after creating the task' -r
complete -c todors -n "__fish_todors_using_subcommand a" -s h -l help -d 'Print help'
complete -c todors -n "__fish_todors_using_subcommand done" -s h -l help -d 'Print help'
complete -c todors -n "__fish_todors_using_subcommand do" -s h -l help -d 'Print help'
complete -c todors -n "__fish_todors_using_subcommand list" -l all -d 'Display all tasks, even the completed ones'
complete -c todors -n "__fish_todors_using_subcommand list" -s h -l help -d 'Print help'
complete -c todors -n "__fish_todors_using_subcommand ls" -l all -d 'Display all tasks, even the completed ones'
complete -c todors -n "__fish_todors_using_subcommand ls" -s h -l help -d 'Print help'
complete -c todors -n "__fish_todors_using_subcommand remove" -s h -l help -d 'Print help'
complete -c todors -n "__fish_todors_using_subcommand rm" -s h -l help -d 'Print help'
complete -c todors -n "__fish_todors_using_subcommand edit" -s h -l help -d 'Print help'
complete -c todors -n "__fish_todors_using_subcommand due" -s h -l help -d 'Print help'
complete -c todors -n "__fish_todors_using_subcommand undone" -s h -l help -d 'Print help'
complete -c todors -n "__fish_todors_using_subcommand undo" -s h -l help -d 'Print help'
complete -c todors -n "__fish_todors_using_subcommand clean" -s h -l help -d 'Print help'
complete -c todors -n "__fish_todors_using_subcommand modify" -l priority -l pri -r
complete -c todors -n "__fish_todors_using_subcommand modify" -l due-date -r
complete -c todors -n "__fish_todors_using_subcommand modify" -l rm-priority -l rm-pri
complete -c todors -n "__fish_todors_using_subcommand modify" -l rm-due-date
complete -c todors -n "__fish_todors_using_subcommand modify" -s h -l help -d 'Print help'
complete -c todors -n "__fish_todors_using_subcommand mod" -l priority -l pri -r
complete -c todors -n "__fish_todors_using_subcommand mod" -l due-date -r
complete -c todors -n "__fish_todors_using_subcommand mod" -l rm-priority -l rm-pri
complete -c todors -n "__fish_todors_using_subcommand mod" -l rm-due-date
complete -c todors -n "__fish_todors_using_subcommand mod" -s h -l help -d 'Print help'
complete -c todors -n "__fish_todors_using_subcommand next" -s h -l help -d 'Print help'
complete -c todors -n "__fish_todors_using_subcommand help; and not __fish_seen_subcommand_from add done list remove edit due undone clean modify next help" -f -a "add" -d 'Add a task to the list'
complete -c todors -n "__fish_todors_using_subcommand help; and not __fish_seen_subcommand_from add done list remove edit due undone clean modify next help" -f -a "done" -d 'Mark selected tasks as done'
complete -c todors -n "__fish_todors_using_subcommand help; and not __fish_seen_subcommand_from add done list remove edit due undone clean modify next help" -f -a "list" -d 'List all the tasks or those that match the query'
complete -c todors -n "__fish_todors_using_subcommand help; and not __fish_seen_subcommand_from add done list remove edit due undone clean modify next help" -f -a "remove" -d 'Remove selected item from the todo file'
complete -c todors -n "__fish_todors_using_subcommand help; and not __fish_seen_subcommand_from add done list remove edit due undone clean modify next help" -f -a "edit" -d 'Edit the todo file with a text editor'
complete -c todors -n "__fish_todors_using_subcommand help; and not __fish_seen_subcommand_from add done list remove edit due undone clean modify next help" -f -a "due" -d 'List all due tasks'
complete -c todors -n "__fish_todors_using_subcommand help; and not __fish_seen_subcommand_from add done list remove edit due undone clean modify next help" -f -a "undone" -d 'Mark selected tasks as not done'
complete -c todors -n "__fish_todors_using_subcommand help; and not __fish_seen_subcommand_from add done list remove edit due undone clean modify next help" -f -a "clean" -d 'Clean all the completed tasks'
complete -c todors -n "__fish_todors_using_subcommand help; and not __fish_seen_subcommand_from add done list remove edit due undone clean modify next help" -f -a "modify" -d 'Modify selected tasks as desired'
complete -c todors -n "__fish_todors_using_subcommand help; and not __fish_seen_subcommand_from add done list remove edit due undone clean modify next help" -f -a "next" -d 'Show the next task to do base on the urgency task sort we have'
complete -c todors -n "__fish_todors_using_subcommand help; and not __fish_seen_subcommand_from add done list remove edit due undone clean modify next help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
