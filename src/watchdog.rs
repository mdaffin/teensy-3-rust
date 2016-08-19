use core::intrinsics::volatile_store;
macro_rules! reg_write { ($x:expr, $t:ty, $v:expr) => ( volatile_store($x as *mut $t, $v) ) }

const WDOG_UNLOCK: u32 = 0x4005200E; // Watchdog Unlock register
const WDOG_STCTRLH: u32 = 0x40052000; // Watchdog Status and Control Register High

pub struct WatchDog {}

impl WatchDog {
    fn unlock() {
        unsafe {
            reg_write!(WDOG_UNLOCK, u16, 0xC520);
            reg_write!(WDOG_UNLOCK, u16, 0xD928);
        }
    }

    pub fn disable() {
        WatchDog::unlock();
        unsafe {
            reg_write!(WDOG_STCTRLH, u16, 0x01D2);
        }
    }
}
