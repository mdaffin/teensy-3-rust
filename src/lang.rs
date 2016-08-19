use core;

#[lang="eh_personality"]
extern "C" fn eh_personality() {}
#[lang="panic_fmt"]
extern "C" fn rust_begin_unwind(_fmt: &core::fmt::Arguments,
                                    _file_line: &(&'static str, usize))
                                    -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() -> () {
    loop {}
}

