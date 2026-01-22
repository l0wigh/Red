# Command list for Red

This file is a bit messy but should contains most of the command available and a little introduction on how to use them. Most of these commands are coming from ed and will work kind of the same.

## Implemented Commands
Every command having a dollar sign means that you can specify a line or sometimes a range

- $a -> insert mode on top of the current line
- $i -> insert mode after the current line
- w -> save the file. can also take an optionnal argument to save to another filepath
- q -> quit Red
- Q -> force quit without saving
- c -> erase the line and put you into insert mode at that position
- $d -> delete the current line
- P -> hide/show prompt
- $? -> this is how you specify a command with a specific line
- $,$? -> this is how you specify a range to apply a command to
- ,$? -> this is a range from your current line to another line
- %? -> apply a command to a whole file
- ,? -> apply a command to the current line
- / -> search command
- s/ -> search and replace command
- $y -> copy the current line to the line number (or range) given. it obviously requires a number and can't be used without one
- $r -> like y command, but replace the line selected instead of pasting the current line on those numbers
- $s/ -> search and replace on the current line, a line number, a range or the whole file (regex)
- j -> go down a line
- k -> go up a line
- e -> edit the current line