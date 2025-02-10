# R(ust)ed(itor)

![screenshot](https://raw.githubusercontent.com/l0wigh/Red/refs/heads/master/red.png)

Red is a clone of the famous Ed file editor from UNIX.

It is NOT Restricted Ed. I learned about this ed version after I choose the name of this project.

Without Ed, no Ex. Without Ex, no Vi. Without Vi, no Vim. Without Vim, no Good Programmers.

## Current state

Basic features coming from Ed are there. Some commands might not work **exactly** like you would expect. I made some choices that feels more natural to me. For example I implemented regex search on the whole file with the `/` command. That is not the way it works in Ed or Ex. I just feel that it's more confortable.

## Build && Installation

- Clone the repo
- Enter the cloned repo folder
- `cargo install --path .`

## Did you wrote this README.md with Red ?

Yes. Yes I did.

And I also do update it with Red.

## This Rust code is horrible...

Yeah I know, that's my first (and maybe last) "real" Rust program. Don't blame me and do pull request if it's required.

## Special Thanks

- [bcheronn](https://github.com/bcheronn): You are the culprit for this.

- [j1mbo64](https://github.com/j1mbo64): He struggled to teach me Rust. Unfortunately I didn't listen to him. That's the reason this code is ugly.