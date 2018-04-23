# telemacher [![Build Status](https://travis-ci.org/attackgoat/telemacher.svg?branch=master)](https://travis-ci.org/attackgoat/telemacher)
"We've got sun, earth, and atmosphere, and when you've got that, you've got weather!" - Harris K. Telemacher

# Compilation Guide

## Prerequisite: The Rust Programming Language Compiler
To install Rust, run the following in your terminal, then follow the onscreen instructions.

```bash
curl https://sh.rustup.rs -sSf | sh
```

[Resources for additional OSes](https://www.rust-lang.org/en-US/install.html)

## Prerequisite: Telemacher source code

```bash
git clone https://github.com/attackgoat/telemacher.git
cd telemacher
```

## Release mode build

```bash
cargo build --release
```

After build the binary will be located at `target/release/telemacher`.

# Usage

Runs a chat server. Press `CTRL + C` or preferrably send a `SIGTERM` to stop.

```bash
USAGE:
    telemacher [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --address <ADDRESS>    Sets a listen address for incoming HTTP messages [default: localhost]
    -p, --port <PORT>          Sets a listen port for incoming HTTP messages [default: 9000]

SUBCOMMANDS:
    flush    Flushes geo and weather cache data
    help     Prints this message or the help of the given subcommand(s)
```

