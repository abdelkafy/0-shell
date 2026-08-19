# 0-Shell

**0-Shell** is a minimalist Unix-like shell written in **Rust**.
The project implements common shell commands from scratch using Rust's system-level abstractions, without relying on external shell utilities such as `bash` or `sh`.

##  Features

* Interactive shell prompt
* Command parsing and execution
* File and directory management
* Unix-style error handling
* `Ctrl+C` (`SIGINT`) handling
* Graceful exit with `Ctrl+D` and `exit`

### Supported Commands

| Command | Description                          |
| ------- | ------------------------------------ |
| `echo`  | Print arguments to standard output   |
| `cd`    | Change the current working directory |
| `ls`    | List directory contents              |
| `pwd`   | Print the current working directory  |
| `cat`   | Display file contents                |
| `cp`    | Copy files and directories           |
| `rm`    | Remove files and directories         |
| `mv`    | Move files and directories           |
| `mkdir` | Create directories                   |
| `exit`  | Exit the shell                       |

### `ls` Options

```text
-l    Long listing format
-a    Show hidden files
-F    Classify file types
```

### `rm` Option

```text
-r    Remove directories recursively
```

##  Requirements

* Rust
* Linux / Unix environment
* Cargo

The project can also be used through **WSL** on Windows.

##  Build

Clone the repository:

```bash
git clone https://learn.zone01oujda.ma/git/ybourazz/0-shell.git
cd 0-shell
```

Build the project:

```bash
cargo build --release
```

## ▶️ Run

```bash
./target/release/0-shell
```

## Example

```text
$ pwd
/home/user

$ mkdir test
$ cd test

$ echo "Hello World"
Hello World

$ ls -la
total 8
drwxr-xr-x 2 user user 4096 Aug 20 12:00 .
drwxr-xr-x 3 user user 4096 Aug 20 12:00 ..

$ cd ..
$ rm -r test

$ exit
```

Unknown commands are handled with:

```text
$ hello
Command 'hello' not found
```

## What This Project Teaches

This project provides practical experience with:

* Unix filesystem operations
* Standard input/output
* File descriptors
* Command parsing
* Process and signal handling
* Rust error handling
* System-level programming
* Working with Unix paths and permissions


## Project Objective

The objective is to understand the fundamental mechanisms behind a Unix shell while implementing its core functionality in Rust.

The shell follows basic Unix conventions while keeping the implementation simple, safe, and self-contained.

