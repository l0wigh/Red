# Command rules discovered in **ed**

Theses rules were found while testing stuff with ed. I'm using them to create RED

## Whole file rule

`%` and `,` will do the command following them.

## No command passed

When no commands are passed, it will print line(s)

Depending of the situation, it will print the current line or the last line selected (when `,` is used)

Example :

% or , = print current line
$, or ,$ = print the line number $
$ = print the line number $
$1,$2 = will print $2

Everytime it print a specific line number it will lock it
