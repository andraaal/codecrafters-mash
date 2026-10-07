# mash

`mash` is a small interactive shell written in Rust with some guidance from [codecrafters.io](codecrafters.io).

It supports running external commands, a handful of builtins, command history,
aliases, piping, and basic output/error redirection.

## Codecrafters
That is a website that breaks the development of larger applications down into some smaller tasks, but gives you no guidance whatsoever on how you should complete the tasks. That is completely up to you. Then when you are finished you push to their git server and they run some automatic tests to verify your progress.

Sadly their tests don't 100% align with what I had in mind, so there is a feature flag that runs this shell in codecrafters-compliant mode. Otherwise it will fail some of their test cases.

That also means that when they push it to your repo after testing you get some pretty useless commit messages

Note: From here on this README is partly AI-generated, but has been reviewed/rewritten by a human.

## Features

- Run external commands with arbitrary arguments
- Builtins:
  - `exit`
  - `echo`
  - `type`
  - `pwd`
  - `cd`
  - `history`
  - `alias`
  - `unalias`
- Pipes with `|`
- Redirection:
  - `>` / `1>` overwrite stdout
  - `>>` / `1>>` append stdout
  - `2>` overwrite stderr
  - `2>>` append stderr
- Persistent history in `.mash_history`
- Alias persistence in `.mash_aliases`
- Command and file completion

## MSRV (Minimum Supported Rust Version)

`mash` has been developed and tested with Rust 1.91. Other versions may work (probably do), but are not officially supported. Newer versions of edition 2024 will pretty much guaranteed work.

## Build and Run

First [install Rust](https://rustup.rs/).
Then get the newest stable toolchain for your system with `rustup install stable`.
Then just use cargo to run it, nothing fancy.

```bash
cargo build
cargo run
```

## Quick Usage
A few examples of what this shell is capable of:

External commands:

```bash
ls -la
cat README.md
```

Builtins:

```bash
pwd
cd src
type echo
history 10
```

Pipes and redirection:

```bash
cat Cargo.toml | grep rustyline
echo hello > out.txt
```

## Builtins

- `exit`
  - Exits `mash`.
- `echo [args...]`
  - Prints arguments joined by spaces.
- `type <name>`
  - Shows whether `<name>` is an alias, a builtin, an executable on `PATH`, or not found.
- `pwd`
  - Prints the current working directory.
- `cd <path>`
  - Changes the current directory.
  - On Unix targets, `~` is expanded to `$HOME`.
- `history [count]`
  - Shows command history, optionally limiting to the last `count` entries.
- `alias <name> <replacement>`
  - Creates or updates an alias.
  - Note: this is space-delimited syntax (`alias ll ls`), not `alias ll='ls'`.
- `unalias <name>`
  - Removes an alias.
