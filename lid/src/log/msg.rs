use crate::time;
use std::ops::Range;
use std::vec::Vec;

// Different log sinks may need to log only given parts of the message.
// That's why it keeps track of its parts.
pub struct LogMsg {
    full: Vec<u8>,
    datetime: Range<usize>,
    pid: Range<usize>,
    tid: Range<usize>,
    level: Range<usize>,
    msg: Range<usize>,
}

impl LogMsg {
    pub(super) fn create(
        dt: time::UtcDatetime,
        pid: u32,
        tid: u32,
        lvl: &str,
        msg_args: std::fmt::Arguments<'_>,
    ) -> LogMsg {
        // TODO: Allocator
        let mut full: Vec<u8> = std::vec::Vec::with_capacity(512);
        let datetimer = Self::append(&mut full, dt);
        full.push(b' ');
        let pidr = Self::append(&mut full, pid);
        full.push(b' ');
        let tidr = Self::append(&mut full, tid);
        full.push(b' ');
        let lvlr = Self::append(&mut full, lvl);
        full.push(b':');
        full.push(b' ');
        let msgr = Self::append(&mut full, msg_args);

        LogMsg {
            full: full,
            datetime: datetimer,
            pid: pidr,
            tid: tidr,
            level: lvlr,
            msg: msgr,
        }
    }

    pub fn full_msg(&self) -> &[u8] {
        self.full.as_slice()
    }

    pub fn datetime(&self) -> &[u8] {
        &self.full[self.datetime.clone()]
    }

    pub fn pid(&self) -> &[u8] {
        &self.full[self.pid.clone()]
    }

    pub fn tid(&self) -> &[u8] {
        &self.full[self.tid.clone()]
    }

    pub fn level(&self) -> &[u8] {
        &self.full[self.level.clone()]
    }

    pub fn msg(&self) -> &[u8] {
        &self.full[self.msg.clone()]
    }

    pub fn is_empty(&self) -> bool {
        self.full.is_empty()
    }

    fn append<T: std::fmt::Display>(buf: &mut Vec<u8>, part: T) -> Range<usize> {
        use std::io::Write;
        let start = buf.len();
        write!(buf, "{}", part).unwrap();
        Range {
            start,
            end: buf.len(),
        }
    }
}
