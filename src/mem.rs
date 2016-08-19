extern "C" {
    static mut _sflashdata: u32;
    static mut _sdata: u32;
    static mut _edata: u32;
    static mut _sbss: u32;
    static mut _ebss: u32;
}

#[inline(always)]
pub fn init() {
    unsafe {
        let mut src: *mut u32 = &mut _sflashdata;
        let mut dest: *mut u32 = &mut _sdata;

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
    }
}
