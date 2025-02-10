# Red Manual

Welcome to the wonderful world of ancient text editors.

I don't know why you are there, but make yourself confortable, grab a cup of hot chocolate, and let me guide you through this.

## Use cases

You probably think : `Why I would use something like this`, and you are right !

Coding ? Pretty much not as useful as anything else.

Writing a book ? Same here.

Creating a new file with some data that you want to paste ? echo "data" > newfile

Well yes, but Red still gives you an advantage : Quick file editing.

Echo is fine, but will not help you deleting an unecessary line.

(Neo)Vim is also fine, but still a bit overkill to just write 2-3 lines.

VSCode ? Running a webpage to change a faulty variable or a bad README line ? Don't use VSCode.

I think you get the point. Red is perfect for small text/code editing. Plus, nothing blocks you from using it as your main editor.

## Your first steps

It's pretty easy to start editing files with Red. Simply start Red with or without a file to edit.

You are now in **command** mode. Your text will not be inserted for now. You'll need to use the 'a' or 'i' command.

'a' will append after the line you currently are (on startup it's the last line of the file).

'i' will insert line before the line you currently are.

You are now in insert mode. Go ahead, type whatever you want. When you are finished, go to a newline and press '.' and then ENTER.

You are back in command mode, can now use 'w' to save your progress.

Keep in mind that if you get a red '?' after using 'w' command, it means you need to pass a filename to it (ex: w myfile).

You can then use 'q' to quit. If 'q' command gives you a red '?', you need to save the file or force quit using 'Q'.

Good job ! You created your first file with Red ! That was easy right ? Right... ?

## Search and Destroy (Replace)

I'm sure you would like to know how to search and replace a mistyped word instead of rewriting the whole line. It's faily easy.

Let's first learn how to search for every occurance of a word.

You can simply use the search command `/` like this : /red

This will print every lines containing the word red.

Obviously you can search for more than just a word. We will see this later.

Now let's see how to change a word with something else on the line you currently are.

To see the line currently "locked" use `p` or `n` command.

You can change line by simply putting the line number and hit enter.

You are on the right line ? You good. Ok now, let's use the `s` command : s/ed/red