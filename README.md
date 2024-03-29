# aidandenlinger's Advent of Code 2022

Doing Advent of Code 2022 in Rust for fun :)

Solutions prioritize legibility over optimality, although both are limited to
the extent of my knowledge ;). You can find Advent of Code 2022
[here](https://adventofcode.com/2022).

## Install
You need the rust toolchain, which you can install with
[rustup](https://rustup.rs/).

## Usage
Each day has two folders - one for part a and one for part b.

To run, optionally place your input in the `input` folder with the filename
`dayxx.txt`, with `xx` being the day you want to run. By default, it runs with
my inputs. Then, open that day and part's folder and run `cargo run`. These
solutions assume proper input, it will panic on improper inputs.

`cargo test` will run basic tests on the examples from the problem prompt, as
well as test that it produces the expected answer from my input. If you change
the inputs, that unit test will now fail since the input is different, this
would be expected.

## Thanks
Thanks to other folks publishing their Rust repos so I can compare and improve
my code afterwards :)

- timvisee, <https://github.com/timvisee/advent-of-code-2022>
- fspoettel, <https://github.com/fspoettel/advent-of-code-2022>
- AxlLind, <https://github.com/AxlLind/AdventOfCode2022>
- aldanor, <https://github.com/aldanor/aoc-2022>
- fasterthanlime, <https://fasterthanli.me/series/advent-of-code-2022>

## Contributing
Not accepting contributions, these are puzzles :)

## License
MIT
