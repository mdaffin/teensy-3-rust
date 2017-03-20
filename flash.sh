#!/bin/sh
set -e
cargo build --target thumbv7em-none-eabi --release
arm-none-eabi-objcopy -O ihex -R .eeprom target/thumbv7em-none-eabi/release/blink target/blink.hex
teensy-loader-cli -v -w --mcu=mk20dx256 target/blink.hex
