# Kroyer

Kroyer is a program used to create random pictures from a set of rules in a grammar file.
It has barely any practical use cases, but can be fun to tinker around with.

## Installation

To install `kroyer`, you can either clone the repo and build it

```cmd
$ git clone https://github.com/AtleSkaanes/kroyer
$ cd kroyer
$ cargo build --release
```

Or install it with `cargo`

```cmd
$ cargo install --git https://github.com/AtleSkaanes/kroyer
```

## Usage

To generate a random image, just run the application. This will generate a image using the default grammar, and output it to `out.png`.
To use a custom grammar file, give the path to the file as an argument. use `--dump-default-grammar` to get the default grammar, which can be a good starting point.

## Inspiration

Kroyer is named after the old danish painter [P.S. Krøyer](https://en.wikipedia.org/wiki/Peder_Severin_Kr%C3%B8yer).
The concept was inspired by Rexims randomart program ([link](https://en.wikipedia.org/wiki/Peder_Severin_Kr%C3%B8yer))
