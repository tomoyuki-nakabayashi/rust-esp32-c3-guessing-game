
```shell
cargo build
esptool.py --chip esp32c3 elf2image target/riscv32imc-esp-espidf/debug/rust-esp32-c3-guessing-game
esptool.py --chip esp32c3 -p /dev/ttyACM0 -b 460800 --before=default_reset --after=hard_reset write_flash --flash_mode dio --flash_freq 80m --flash_size 2MB 0x10000 target/riscv32imc-esp-espidf/debug/rust-esp32-c3-guessing-game.bin
```