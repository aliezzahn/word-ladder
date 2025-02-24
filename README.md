# Word Ladder

![CI](https://github.com/aliezzahn/word-ladder/actions/workflows/ci.yml/badge.svg)
![CD](https://github.com/aliezzahn/word-ladder/actions/workflows/cd.yml/badge.svg)

A Rust implementation of the Word Ladder problem.

## Problem Description

Given two words (`begin_word` and `end_word`), and a dictionary of words (`word_list`), find the length of the shortest transformation sequence from `begin_word` to `end_word` such that:

- Only one letter can be changed at a time.
- Each transformed word must exist in the `word_list`.

## Usage

### Installation

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
word-ladder = "0.1.0"
```

### Example

```rust
use word_ladder::ladder_length;

fn main() {
    let begin_word = String::from("hit");
    let end_word = String::from("cog");
    let word_list = vec![
        String::from("hot"),
        String::from("dot"),
        String::from("dog"),
        String::from("lot"),
        String::from("log"),
        String::from("cog"),
    ];
    let result = ladder_length(begin_word, end_word, word_list);
    println!("Transformation steps: {}", result); // Output: 5
}
```

## Running Tests

To run the tests, use the following command:

```bash
cargo test
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
