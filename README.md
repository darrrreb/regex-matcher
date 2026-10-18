# Brzozowski Regex Matcher

A basic regular expression matcher in Rust, based on Brzozowski derivatives learned in university.
This is a practice project for learning Rust, written by hand.

## How it works

To match a string, the matcher takes the derivative of the regex with respect to each character, simplifying after every
step. If the final regex can match the empty string, the input matches.

Supports the empty language (`Zero`), the empty string (`One`), single characters, alternation, sequences, Kleene star
and bounded repetition (`NTimes`).

This could quite easily be extended to more complex regex expressions, such as ? `?` or `~`.

The code could also be simplified to add functions for building the regular expressions, or even parsing from strings.

## Running

```
cargo run --release
```

This runs a small example and times the matcher on two classic "evil" regexes, `(a?){n} a{n}` and `(a*)* b`, which take
exponential time in backtracking matchers (such as Python's).

## Testing

```
cargo test --release
```