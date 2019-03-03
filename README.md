# Perfect Good Bad Guessing Game

Adapted from [Sharpen Your Saw VI](https://shackbarth.github.io/sharpen-your-saw/#/).

This is a number guessing game.
The game will pick a three digit number. The player will provide a three digit guess.
The game will provide feedback to the user for each guess.

## Rules

- If a digit is correct and in the correct spot, it is perfect
- If a digit is correct but not in the right place, it is good
- If a digit is not correct, it is bad
- The game ends when the player's guess is perfect

## Usage

First, install Rust using [rustup.rs](https://rustup.rs/)

```
USAGE:
    perfect-good-bad [FLAGS] [OPTIONS]

FLAGS:
    -a, --auto       Asks the CPU to play itself
        --debug      Turns on debug logging
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --digit <digit>    Sets the number of digits used for the secret number [default: 3]
```
