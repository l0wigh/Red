# R(ust)ed(itor)

![screenshot](https://raw.githubusercontent.com/l0wigh/Red/refs/heads/master/red0-3-0.png)

Red is a clone of the famous Ed file editor from UNIX.

It is NOT Restricted Ed. I learned about this ed version after I choose the name of this project.

Without Ed, no Ex. Without Ex, no Vi. Without Vi, no Vim. Without Vim, no Good Programmers.

## Current state

Basic features coming from Ed are there. Some commands might not work **exactly** like you would expect. I made some choices that feels more natural to me. For example I implemented regex search on the whole file with the `/` command. That is not the way it works in Ed or Ex. I just feel that it's more confortable.

Since version 0.4.0, Red uses rustyline wich gives a better "REPL" experience. You can use arrows to go up and down in the history, and also use left/right arrows to edit the line you are working on. This is a huge step-up, and it now feels way more useable for small coding sessions. It still not as good as a full fledge modern editor, but it's not a pain to use any more.

## Build && Installation

You obviously need the rust toolchain installed on your machine.

`git clone https://github.com/L0Wigh/Red && cd Red && cargo install --path .`

If `~/.cargo/bin` is in your PATH, you now can run `red` to start having fun.

## Did you wrote this README.md with Red ?

Yes. Yes I did.

And I also do update it with Red.

## This Rust code is horrible...

Yeah I know, that's my first (and maybe last) "real" Rust program. Don't blame me and do pull request if it's required.

## Special Thanks

- [bcheronn](https://github.com/bcheronn): You are the culprit for this.

- [j1mbo64](https://github.com/j1mbo64): He struggled to teach me Rust. Unfortunately I didn't listen to him. That's the reason this code is ugly.