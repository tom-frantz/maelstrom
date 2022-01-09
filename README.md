# Maelstrom 

## Dependencies
- [Rust](https://www.rust-lang.org/tools/install)
- [mdBook](https://rust-lang.github.io/mdBook/guide/installation.html)

## How To Run
```shell
# To run the game; This is for all the people who want to play!
cargo run --package maelstrom --bin maelstrom

# To run the demo binary; This is a 'staged' version of the game to show off to people who don't want to play it.
cargo run --package maelstrom --bin demo

# to run the test binary; used primarily for testing hot parts of the code.
cargo run --package maelstrom --bin test
```

## Useful commands
```shell
# To read what little documentation I've added in so far.
cargo doc --open

# To read the book
mdbook serve --open book
```
