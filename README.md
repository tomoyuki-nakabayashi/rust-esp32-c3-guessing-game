## Rust Guessing Game on ESP32-C3 with std

Run Guessing Game in The Rust Programming Language on ESP32-C3 with Rust std library based on [rust-esp32-std-demo](https://github.com/ivmarkov/rust-esp32-std-demo).

## How to build & run

### build

Detailed instructions are in original repository [rust-esp32-std-demo](https://github.com/ivmarkov/rust-esp32-std-demo).

In briefly

- install ESP-IDF v4.3.1 and setup environment with `get_idf`

clang11+ (for Ubuntu)

```shell
bash -c "$(wget -O - https://apt.llvm.org/llvm.sh)"
```

Install related tools:

```
cargo install ldproxy
cargo install espflash
```

Get Rust source:

```
rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
```

Build with nightly toolchain:

```
rustup install nightly

git clone https://github.com/tomoyuki-nakabayashi/rust-esp32-c3-guessing-game.git
cd rust-esp32-c3-guessing-game.git
rustup override add nightly
```

### run

Flash (`ttyACM0` depends on your environment or target board):

```
espflash /dev/ttyACM0 target/riscv32imc-esp-espidf/debug/rust-esp32-c3-guessing-game
```

Use ESP-IDF bandled monitor tool:

```
# at some ESP-IDF project
idf.py monitor
```

Then, you can enjoy the Guessing Game.

```shell
$ idf.py monitor
# snip
Guess the number!
Please input your guess.
You guessed: 50
Too small!
Please input your guess.
You guessed: 75
Too big!
Please input your guess.
You guessed: 67
Too big!
Please input your guess.
You guessed: 58
Too big!
Please input your guess.
You guessed: 55
Too small!
Please input your guess.
You guessed: 56
You win!
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
