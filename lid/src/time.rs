use std::fmt;
use std::mem::MaybeUninit;

// Very simple timestamp, timestamp -> date-time utilities base on some libc functions.
// The reasons for creating and using these utilities are the following:
// 1. The timestamp obtained from the std::time::SystemTime::now() can be easily/directly converted
// to date-time string representation
// 2. Using external crates, such as `chrono`, means adding dependencies for something as simple
// as this one
// 3. One of the simplest usages of the `chrono` library does 2 allocations of 16 bytes each,
// followed by two reallocations to 48 bytes and two deallocations:
// let now = chrono::Utc::now();
// write!(out, "{}", now.format("%Y-%m-%d %H:%M:%S%.6f")).unwrap();
// 4. The above functionality simulated with the functions from this module and then calling
// write!(out, "{}-{}...", ...); does no memory allocations and is about 200-300 ns faster.

// This is a bit of a hack but this actually provides all of the needed functionality for an OS
// error
type OsError = std::io::Error;

pub struct UtcTime {
    pub sec: i64,
    pub usec: i32,
}

pub struct UtcDatetime {
    pub year: i32,
    pub mon: i32,
    pub day: i32,
    pub hour: i32,
    pub min: i32,
    pub sec: i32,
    pub usec: i32,
}

impl UtcTime {
    pub fn now() -> Result<UtcTime, OsError> {
        let mut ts = MaybeUninit::<libc::timeval>::uninit();
        let rc = unsafe { libc::gettimeofday(ts.as_mut_ptr(), std::ptr::null_mut()) };
        if rc == 0 {
            let ts = unsafe { ts.assume_init() };
            Ok(UtcTime {
                sec: ts.tv_sec,
                usec: ts.tv_usec as i32,
            })
        } else {
            Err(OsError::last_os_error())
        }
    }

    pub fn to_datetime(&self) -> Result<UtcDatetime, OsError> {
        let mut tm = MaybeUninit::<libc::tm>::uninit();
        let ts: libc::time_t = self.sec;
        let res = unsafe { libc::gmtime_r(&ts, tm.as_mut_ptr()) };
        if !res.is_null() {
            let tm = unsafe { tm.assume_init() };
            Ok(UtcDatetime {
                year: 1900 + tm.tm_year,
                mon: 1 + tm.tm_mon,
                day: tm.tm_mday,
                hour: tm.tm_hour,
                min: tm.tm_min,
                sec: tm.tm_sec,
                usec: self.usec,
            })
        } else {
            Err(OsError::last_os_error())
        }
    }
}

impl UtcDatetime {
    pub fn now() -> Result<UtcDatetime, OsError> {
        UtcTime::now().and_then(|ts| ts.to_datetime())
    }
}

impl fmt::Display for UtcTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{:06}", self.sec, self.usec)
    }
}

impl fmt::Debug for UtcTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for UtcDatetime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:06}",
            self.year, self.mon, self.day, self.hour, self.min, self.sec, self.usec
        )
    }
}

impl fmt::Debug for UtcDatetime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
