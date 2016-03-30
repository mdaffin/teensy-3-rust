#![feature(lang_items,core_intrinsics,asm,start)]
#![no_std]
#![crate_type="staticlib"]

use core::intrinsics::{volatile_store};

//#[lang="stack_exhausted"] extern fn stack_exhausted() {}
#[lang="eh_personality"] extern fn eh_personality() {}
#[lang="panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_unwind(_fmt: &core::fmt::Arguments, _file_line: &(&'static str, usize)) -> !
{
    loop {}
}

#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr0() -> ()
{
    loop {}
}

#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr1() -> ()
{
    loop {}
}

macro_rules! GPIOC_PDOR   {() => (0x400FF080 as *mut u32);} // GPIOC_PDOR - page 1334,1335
macro_rules! WDOG_UNLOCK  {() => (0x4005200E as *mut u16);} // Watchdog Unlock register
macro_rules! WDOG_STCTRLH {() => (0x40052000 as *mut u16);} // Watchdog Status and Control Register High
macro_rules! GPIO_CONFIG  {() => (0x40048038 as *mut u32);}
macro_rules! PORTC_PCR5   {() => (0x4004B014 as *mut u32);} // PORTC_PCR5 - page 223/227
macro_rules! GPIOC_PDDR   {() => (0x400FF094 as *mut u32);} // GPIOC_PDDR - page 1334,1337
macro_rules! GPIOC_PDOR   {() => (0x400FF080 as *mut u32);} // GPIOC_PDOR - page 1334,1335

extern {
    static mut _sflashdata: u32;
    static mut _sdata: u32;
    static mut _edata: u32;
    static mut _sbss: u32;
    static mut _ebss: u32;
    fn _estack();
}

#[link_section=".vectors"]
#[allow(non_upper_case_globals)]
#[no_mangle]
pub static ISRVectors: [Option<unsafe extern fn()>; 16] = [
  Some(_estack),          // Stack pointer
  Some(startup),          // Reset
  Some(isr_nmi),          // NMI
  Some(isr_hardfault),    // Hard Fault
  Some(isr_mmfault),      // CM3 Memory Management Fault
  Some(isr_busfault),     // CM3 Bus Fault
  Some(isr_usagefault),   // CM3 Usage Fault
  Some(isr_reserved_1),   // Reserved - Used as NXP Checksum
  None,                   // Reserved
  None,                   // Reserved
  None,                   // Reserved
  Some(isr_svcall),       // SVCall
  Some(isr_debugmon),     // Reserved for debug
  None,                   // Reserved
  Some(isr_pendsv),       // PendSV
  Some(isr_systick),      // SysTick
];

#[link_section=".flashconfig"]
#[allow(non_upper_case_globals)]
#[no_mangle]
pub static flashconfigbytes: [usize; 4] = [
    0xFFFFFFFF,
    0xFFFFFFFF,
    0xFFFFFFFF,
    0xFFFFFFFE,
];

pub unsafe extern fn startup() {
    let mut src: *mut u32 = &mut _sflashdata;
    let mut dest: *mut u32 = &mut _sdata;

    volatile_store(WDOG_UNLOCK!(), 0xC520);
    volatile_store(WDOG_UNLOCK!(), 0xD928);
    volatile_store(WDOG_STCTRLH!(), 0x01D2);

    while dest < &mut _edata as *mut u32 {
        *dest = *src;
        dest = ((dest as u32) + 4) as *mut u32;
        src = ((src as u32) + 4) as *mut u32;
    }

    dest = &mut _sbss as *mut u32;

    while dest < &mut _edata as *mut u32 {
        *dest = 0;
        dest = ((dest as u32) + 4) as *mut u32;
    }

    // Enable system clock on all GPIO ports - page 254
    *GPIO_CONFIG!() = 0x00043F82; // 0b1000011111110000010
    // Configure the led pin
    *PORTC_PCR5!() = 0x00000143; // Enables GPIO | DSE | PULL_ENABLE | PULL_SELECT - page 227
    // Set the led pin to output
    *GPIOC_PDDR!() = 0x20; // pin 5 on port c

    rust_loop();
}

pub fn led_on() {
    unsafe {
        volatile_store(GPIOC_PDOR!(), 0x20);
    }
}

pub fn led_off() {
    unsafe {
        volatile_store(GPIOC_PDOR!(), 0x0);
    }
}

pub fn delay(ms: i32) {
    for _ in 0..ms*250 {
        unsafe {
            asm!("NOP");
        }
    }
}

pub fn rust_loop() {
    loop {
        led_on();
        delay(1000);
        led_off();
        delay(1000);
    }
}

#[start]
fn lang_start(_: isize, _: *const *const u8) -> isize {
    unsafe {
        startup();
    }
    0
}


pub unsafe extern fn isr_nmi() { loop {} }
pub unsafe extern fn isr_hardfault() { loop {} }
pub unsafe extern fn isr_mmfault() { loop {} }
pub unsafe extern fn isr_busfault() { loop {} }
pub unsafe extern fn isr_usagefault() { loop {} }
pub unsafe extern fn isr_reserved_1() { loop {} }
pub unsafe extern fn isr_svcall() { loop {} }
pub unsafe extern fn isr_debugmon() { loop {} }
pub unsafe extern fn isr_pendsv() { loop {} }
pub unsafe extern fn isr_systick() { loop {} }
