## Rust Guessing Game on ESP32-C3 with std

Run Guessing Game in The Rust Programming Language on ESP32-C3 with Rust std library based on [rust-esp32-std-demo](https://github.com/ivmarkov/rust-esp32-std-demo).

## How to build and run

### build

Detailed instructions are in original repository [rust-esp32-std-demo](https://github.com/ivmarkov/rust-esp32-std-demo).

In briefly

- install ESP-IDF v4.3.1 and setup environment with `get_idf`

clang11+ (for Ubuntu)

```shell
bash -c "$(wget -O - https://apt.llvm.org/llvm.sh)"
```

Install related tools:

```console
cargo install ldproxy
cargo install espflash
```

Build with nightly toolchain. Note that this repository contains `rust-toolchain.toml`
file so `rustup` will automatically install a specific version of nightly toolchain
with rust-src component.

```console
git clone https://github.com/tomoyuki-nakabayashi/rust-esp32-c3-guessing-game.git
cd rust-esp32-c3-guessing-game

# Build it. This will also install a nightly toolchain with rust-src component:
cargo build
```

### run

Flash (`ttyACM0` depends on your environment or target board):

```console
espflash /dev/ttyACM0 target/riscv32imc-esp-espidf/debug/rust-esp32-c3-guessing-game
```

Use ESP-IDF bundled monitor tool:

```console
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
