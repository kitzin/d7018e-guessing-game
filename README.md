# Guessing game
Rust tutorial book guessing game with some modification.
## Building and running
1. Build with cargo for release `cargo build --release`, and without release `cargo build`
2. Running `./target/{release,debug}/guesing-game`.
##### Or
1. Building and running with cargo `cargo run` or with release `cargo run --release`

## Output explanation
Reason why the output is out of order is because the history is inside an `HashMap` and that collection does not store its items in order.
