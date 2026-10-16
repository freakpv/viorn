use std::io;

use super::msg::LogMsg;

pub trait LogSink {
    fn log(&mut self, msg: &LogMsg) -> io::Result<()>;
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// A functionality for self-rotating file where it wont go above given max size.
// Once the max size is reached part of the beginning of the file is removed and the writing at
// the end continues.
pub struct SlidingLogFile {
    file: std::fs::File,
    cur_file_size: u64,
    rem_file_size: u64,
    max_file_size: u64,
}

impl SlidingLogFile {
    pub fn open(
        file_path: &std::path::Path,
        max_file_size: u64,
        rem_file_size: u64,
    ) -> io::Result<SlidingLogFile> {
        std::assert!(max_file_size > 0);
        std::assert!(0 < rem_file_size && rem_file_size <= max_file_size / 2);

        use std::fs::OpenOptions;
        let file = OpenOptions::new().append(true).open(file_path)?;
        let meta = file.metadata()?;

        Ok(SlidingLogFile {
            file: file,
            cur_file_size: meta.len() as u64,
            rem_file_size: rem_file_size,
            max_file_size: max_file_size,
        })
    }
}

impl LogSink for SlidingLogFile {
    fn log(&mut self, msg: &LogMsg) -> io::Result<()> {
        if self.cur_file_size >= self.max_file_size {
            let rem_min_size =
                (self.cur_file_size - self.max_file_size) + msg.full_msg().len() as u64;
            let rem_size = std::cmp::min(self.rem_file_size, rem_min_size);
            use crate::fsext::FileExt;
            self.file.remove_range(0, rem_size as i64)?;
            self.cur_file_size -= rem_size;
        }
        use std::io::Write;
        self.file.write(msg.full_msg()).map(|_| ())
    }
}
