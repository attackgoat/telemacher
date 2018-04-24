# telemacher [![Build Status](https://travis-ci.org/attackgoat/telemacher.svg?branch=master)](https://travis-ci.org/attackgoat/telemacher)

"We've got sun, earth, and atmosphere, and when you've got that, you've got weather!" - Harris K. Telemacher

## Compilation Guide

### Prerequisite: The Rust Programming Language Compiler

To install Rust, run the following in your terminal, then follow the onscreen instructions.

```bash
curl https://sh.rustup.rs -sSf | sh
```

[Resources for additional OSes](https://www.rust-lang.org/en-US/install.html)

### Prerequisite: Telemacher source code

```bash
git clone https://github.com/attackgoat/telemacher.git
cd telemacher
```

### Release mode build

```bash
cargo build --release
```

After build the binary will be located at `target/release/telemacher`.

## Usage

Runs a chat server. Press `CTRL + C` or preferrably send a `SIGTERM` to stop.

```bash
USAGE:
    telemacher [OPTIONS] --dark-sky-api-key <KEY> --google-api-key <KEY> [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --address <ADDRESS>         Sets the HTTP listen address [default: 0.0.0.0]
    -d, --dark-sky-api-key <KEY>    Sets the API key used for Dark Sky services
    -g, --google-api-key <KEY>      Sets the API key used for Google services
    -p, --port <PORT>               Sets the HTTP listen port number [default: 9000]
    -t, --training <FILE>           Sets the (json) training file [default: trained-assistant.json]

SUBCOMMANDS:
    flush    Flushes geo and weather cache data
    help     Prints this message or the help of the given subcommand(s)

```
## Features

I have invented an electic Harris K. Telemacher that tells you what the weather will be like at any place or time. Simply ask Harris (via his nifty REST interface) and you'll be delighted at his witty and increasingly sentient behaivor. Rest assured, Harris is only programmed to respond in a friendly manner and needs no regular servicing.

When you ask Harris something, he will attempt to respond with the appropriate level of specificity. You may ask about a weather condition in general or you may add a date/time or range to your query.

Harris is built upon a capable engine but is deployed with only minimal training. For best results use the following query patterns:

- Are there rains in Spain?
- Is it always sunny in Philadelphia?
- Is it snowing in the Himalayas?
- Is it snowing in Death Valley?
- Will it snow in New York this December?
- What does it feel like in Jamiaca right now?
- Is it humid in Paris?
- Will it be humid in Paris at 4:15PM?
- Will it be humid in Paris at 11PM?
- Will it be humid in Paris at tomorrow?
- How windy is Chicago?
- What was the weather like November 22nd 1963 in Dallas Texas?

You'll quickly find that Harris has not travelled well and does not know of major places, such as Atlanta.

### Weather condition keywords

#### Hail

- hail
- hailing

#### Humidity

- humid

#### Precipitation

- storm
- stormy
- rain
- rainfall
- rainy

#### Snow

- blizzard
- snow
- snowfall
- snowing
- snowstorm
- snowy

#### UV Report

- cloud
- cloudi
- overcast
- depress
- fog
- foggy
- sun
- sunni
- hot
- be sunni

#### Wind

- wind
- windy

## Footnotes

A number of external projects with compatible licenses have been linked into this project. These include the items listed under `[dependencies]` in `/Cargo.toml` and the full un-edited text of `trained-assistant.json` from the Snips NLU library. The provided training material is rather basic and only understands a few queries and localities/spellings, this should be trained properly for actual use.

[![Harris K. Telemacher](http://img.youtube.com/vi/JwhiB4YY640/0.jpg)](http://www.youtube.com/watch?v=JwhiB4YY640)