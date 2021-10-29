#!/usr/bin/fish

#
# To execute:
#   rofi -modi "todo-txt:rofi-todo-txt.fish" -show todo-txt
#


if test "$argv" = "Quit"
    exit 0
end

set info_parts (string split ":" $ROFI_INFO)

if [ "$info_parts[1]" = "TCS" ]
    todo-txt do $info_parts[2]
    exit 0

else if [ "$info_parts[1]" = "CIO" ]
    todo-txt clock $info_parts[2]
    exit 0

else if [ "$info_parts[1]" = "DEL" ]
    todo-txt rm $info_parts[2]
    exit 0
end

if test $ROFI_RETV = 1
    echo -en "\0prompt\x1fSelect action\n"

    set trimmed (string trim -l -r $argv)
    set parts (string split ":" $trimmed)
    set number $parts[1]
    set display_task (string trim -l -r $parts[2..])

    echo -en "$display_task\0nonselectable\x1ftrue\n"
    echo -en "Toggle Complete State\0info\x1fTCS:$number\n"
    echo -en "Check In/Out\0info\x1fCIO:$number\n"
    echo -ne "Delete\0info\x1fDEL:$number\n"

    echo "Quit"

    exit 0

else if test $ROFI_RETV = 2
    set parts (string split " " $argv)

    if test $parts[1] = "add"
	todo-txt add "$parts[2..]" > /dev/null
	exit 0
    else
	echo "Unknown special command, $argv"
	exit 1
    end
end

echo -en "\0prompt\x1fSelect todo\n"

eval todo-txt ls
