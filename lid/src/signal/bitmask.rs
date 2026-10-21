use std::ffi::{c_char, c_int, CStr};
use std::fmt;

// This C function is not provided by the Rust libc crate.
#[cfg_attr(target_env = "gnu", link(name = "c"))]
unsafe extern "C" {
    fn sigdescr_np(sig: c_int) -> *const c_char;
}

pub fn signal_description(sig: u32) -> Option<&'static str> {
    let ptr = unsafe { sigdescr_np(sig as i32) };
    if !ptr.is_null() {
        // The function returns a statically allocated string.
        unsafe { CStr::from_ptr(ptr) }.to_str().ok()
    } else {
        None
    }
}

#[derive(Debug)]
pub struct Bitmask {
    pub bits: u32,
}

impl Bitmask {
    pub fn bits(&self) -> u32 {
        self.bits
    }
    pub fn contains(&self, sig: u32) -> bool {
        (self.bits & (1u32 << sig)) != 0
    }
}

impl fmt::Display for Bitmask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut bits = self.bits;
        let mut sep = "";
        while bits != 0 {
            let sig_str = signal_description(bits.trailing_zeros()).unwrap_or("UNKNOWN");
            write!(f, "{}{}", sep, sig_str)?;
            sep = "|";
            bits &= bits - 1;
        }
        Ok(())
    }
}
