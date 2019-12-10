
use core::sync::atomic::{AtomicBool, Ordering};

const WDTCTL_ADDRESS: usize = 0x4000_480C;

const WDTPW_SHIFT: u8 = 8;
const WDTPW_MASK: u16 = 0xFF << WDTPW_SHIFT;
const WDTPW_WRITE: u16 = 0x5A << WDTPW_SHIFT;
const WDTPW_READ: u16 = 0x69 << WDTPW_SHIFT;

const WDTHOLD_SHIFT: u8 = 7;
const WDTHOLD_MASK: u16 = 1 << WDTHOLD_SHIFT;

static mut WDT_A_IN_USE: AtomicBool = AtomicBool::new(false);

pub struct WatchdogTimer {
    _unused: ()
}

impl WatchdogTimer {
    pub fn acquire() -> Option<Self> {
        let in_use = unsafe {
            WDT_A_IN_USE.swap(true, Ordering::Relaxed)
        };

        if in_use {
            return None;
        }

        Some(WatchdogTimer{_unused: ()})
    }

    pub fn disable(&mut self) {
        let wdt_ctl = WDTCTL_ADDRESS as *mut u16;

        unsafe {
            let mut value = core::ptr::read_volatile(wdt_ctl);
            value &= !WDTPW_MASK;
            value |= WDTPW_WRITE | WDTHOLD_MASK;
            core::ptr::write_volatile(wdt_ctl, value);
        }
    }

    pub fn enable(&mut self) {
        let wdt_ctl = WDTCTL_ADDRESS as *mut u16;

        unsafe {
            let mut value = core::ptr::read_volatile(wdt_ctl);
            value &= !WDTPW_MASK & !WDTHOLD_MASK;
            value |= WDTPW_WRITE | WDTHOLD_MASK;
            core::ptr::write_volatile(wdt_ctl, value);
        }
    }
}

impl Drop for WatchdogTimer {
    fn drop(&mut self) {
        unsafe {
            WDT_A_IN_USE.store(false, Ordering::Relaxed);
        }
    }
}