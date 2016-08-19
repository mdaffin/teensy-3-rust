extern "C" {
    fn _estack();
    fn main();
}

#[link_section=".vectors"]
#[allow(non_upper_case_globals)]
#[no_mangle]
pub static ISRVectors: [Option<unsafe extern "C" fn()>; 16] = [Some(_estack), // Stack pointer
                                                               Some(main), // Reset
                                                               Some(isr_nmi), // NMI
                                                               Some(isr_hardfault), // Hard Fault
                                                               Some(isr_mmfault), /* CM3 Memory Management Fault */
                                                               Some(isr_busfault), /* CM3 Bus Fault */
                                                               Some(isr_usagefault), /* CM3 Usage Fault */
                                                               Some(isr_reserved_1), /* Reserved - Used as NXP Checksum */
                                                               None, // Reserved
                                                               None, // Reserved
                                                               None, // Reserved
                                                               Some(isr_svcall), // SVCall
                                                               Some(isr_debugmon), /* Reserved for debug */
                                                               None, // Reserved
                                                               Some(isr_pendsv), // PendSV
                                                               Some(isr_systick) /* SysTick */];

#[link_section=".flashconfig"]
#[allow(non_upper_case_globals)]
#[no_mangle]
pub static flashconfigbytes: [usize; 4] = [0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFE];

pub unsafe extern "C" fn isr_nmi() { loop {} }
pub unsafe extern "C" fn isr_hardfault() { loop {} }
pub unsafe extern "C" fn isr_mmfault() { loop {} }
pub unsafe extern "C" fn isr_busfault() { loop {} }
pub unsafe extern "C" fn isr_usagefault() { loop {} }
pub unsafe extern "C" fn isr_reserved_1() { loop {} }
pub unsafe extern "C" fn isr_svcall() { loop {} }
pub unsafe extern "C" fn isr_debugmon() { loop {} }
pub unsafe extern "C" fn isr_pendsv() { loop {} }
pub unsafe extern "C" fn isr_systick() { loop {} }
