use crate::time;
use std::ops::Range;
use std::vec::Vec;

// Different log sinks may need to log only given parts of the message.
// That's why it keeps track of its parts.
pub(super) struct LogMsg {
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
        let append = |buf: &mut Vec<u8>, part| {
            use std::io::Write;
            let start = buf.len();
            write!(buf, "{}", dt).unwrap();
            return Range {
                start: start,
                end: buf.len(),
            };
        };

        // TODO: Allocator
        let mut full: Vec<u8> = std::vec::Vec::with_capacity(512);
        let datetimer = append(&mut full, dt);
        full.push(b' ');
        let pidr = append(&mut full, pid);
        full.push(b' ');
        let tidr = append(&mut full, tid);
        full.push(b' ');
        let lvlr = append(&mut full, lvl);
        full.push(b':');
        full.push(b' ');
        let msgr = append(&mut full, msg_args);

        LogMsg {
            full: full,
            datetime: datetimer,
            pid: pidr,
            tid: tidr,
            level: lvlr,
            msg: msgr,
        }
    }

    pub(super) fn full_msg(&self) -> &[u8] {
        self.full
    }

    pub(super) fn is_empty(&self) -> bool {
        self.full.is_empty()
    }
}
