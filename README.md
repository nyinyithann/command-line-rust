# Learning Rust with Command-Line Rust Book

Some minor differences in code:
- The book uses `clap 2.8`. I am using `clap 4.1.4`.

## What I learnt:
### Chapter 1
- [`std::process::Command`](https://doc.rust-lang.org/std/process/struct.Command.html)
- [`assert_cmd`](https://docs.rs/assert_cmd/latest/assert_cmd/#) crate 
- `$ cargo add --dev assert_cmd`

### Chapter 2
- [`std::env`](https://doc.rust-lang.org/std/env/index.html)
- [`predicates`](https://docs.rs/predicates/latest/predicates/) crate
- `$ cargo run -- -n A B` # separate cargo option with --, all the args behind -- are for the running program
- `$ cargo add clap@4.1.4`
- `$ cargo add --dev predicates`
- prefix 'dies' for test expecting to fail under that given conditions e.g. `dies_no_args`
- `$ cargo test dies`  to run all the tests with names containing the string _dies_
- As Ted Nelson says, “The good news about computers is that they do what you tell them to do. The bad news is that they do what you tell them to do.”
- `println!("{:#?}", vec![1,2,3])` // `#?` for pretty printing
- use `?` instead of `unwrap` on `Result` type to unpack an `Ok` value or propagate the `Err` value to the return type
