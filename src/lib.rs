#![feature(lang_items,core_intrinsics,asm,start)]
#![no_std]
#![crate_type="staticlib"]

pub mod lang;
pub mod isr;
pub mod mem;

use core::intrinsics::volatile_store;

const GPIOC_PDOR: u32 = 0x400FF080; // GPIOC_PDOR - page 1334,1335
const WDOG_UNLOCK: u32 = 0x4005200E; // Watchdog Unlock register
const WDOG_STCTRLH: u32 = 0x40052000; // Watchdog Status and Control Register High
const GPIO_CONFIG: u32 = 0x40048038;
const PORTC_PCR5: u32 = 0x4004B014; // PORTC_PCR5 - page 223/227
const GPIOC_PDDR: u32 = 0x400FF094; // GPIOC_PDDR - page 1334,1337

macro_rules! reg_write { ($x:expr, $t:ty, $v:expr) => ( volatile_store($x as *mut $t, $v) ) }

pub fn init() {
    unsafe {
        reg_write!(WDOG_UNLOCK, u16, 0xC520);
        reg_write!(WDOG_UNLOCK, u16, 0xD928);
        reg_write!(WDOG_STCTRLH, u16, 0x01D2);

	mem::init();

        // Enable system clock on all GPIO ports - page 254
        reg_write!(GPIO_CONFIG, u32, 0x00043F82); // 0b1000011111110000010
        // Configure the led pin
        reg_write!(PORTC_PCR5, u32, 0x00000143); // Enables GPIO | DSE | PULL_ENABLE | PULL_SELECT - page 227
        // Set the led pin to output
        reg_write!(GPIOC_PDDR, u32, 0x20);
    }
}

pub fn led_on() {
    unsafe {
        reg_write!(GPIOC_PDOR, u32, 0x20);
    }
}

pub fn led_off() {
    unsafe {
        reg_write!(GPIOC_PDOR, u32, 0x0);
    }
}

pub fn delay(ms: i32) {
    for _ in 0..ms * 250 {
        unsafe {
            asm!("NOP");
        }
    }
}
