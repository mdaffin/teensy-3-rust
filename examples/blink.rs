#![feature(lang_items,start)]
#![no_std]
#![crate_type="staticlib"]
extern crate teensy;

// Entry point for this program
#[start]
pub fn start(_argc: isize, _argv: *const *const u8) -> isize {
    teensy::init();
    loop {
        teensy::led_on();
        teensy::delay(10000);
        teensy::led_off();
        teensy::delay(10000);
    }
}

