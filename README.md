# Rust on the teensy 3.1

A bare metal example of blink written in rust for the teensy 3.1

## Requirements

* rustc nightly
* cargo
* arm-none-eabi-gcc
* arm-none-eabi-ar
* arm-none-eabi-objcopy

## Compile and upload

```bash
cargo build --target thumbv7em-none-eabi --example blink
arm-none-eabi-objcopy -O ihex -R .eeprom target/thumbv7em-none-eabi/debug/examples/blink blink.hex

echo "Reset teensy now"
teensy-loader-cli -w --mcu=mk20dx256 blink.hex
```
