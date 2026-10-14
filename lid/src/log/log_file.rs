use std::io;

// A functionality for self-rotating file where it wont go above given max size.
// Once the max size is reached part of the beginning of the file is removed and the writing at
// the end continues.
pub(super) struct LogFile {
    file: std::fs::File,
    cur_file_size: u64,
    rem_file_size: u64,
    max_file_size: u64,
}

impl LogFile {
    pub(super) fn open(
        file_path: &std::path::Path,
        max_file_size: u64,
        rem_file_size: u64,
    ) -> io::Result<LogFile> {
        std::assert!(max_file_size > 0);
        std::assert!(0 < rem_file_size && rem_file_size <= max_file_size / 2);

        use std::fs::OpenOptions;
        let file = OpenOptions::new().append(true).open(file_path)?;
        let meta = file.metadata()?;

        Ok(LogFile {
            file: file,
            cur_file_size: meta.len() as u64,
            rem_file_size: rem_file_size,
            max_file_size: max_file_size,
        })
    }

    pub(super) fn metadata(&self) -> io::Result<std::fs::Metadata> {
        self.file.metadata()
    }

    pub(super) fn append(&mut self, msg: &[u8]) -> io::Result<()> {
        if self.cur_file_size >= self.max_file_size {
            let rem_min_size = (self.cur_file_size - self.max_file_size) + msg.len() as u64;
            let rem_size = std::cmp::min(self.rem_file_size, rem_min_size);
            use crate::fsext::FileExt;
            self.file.remove_range(0, rem_size as i64)?;
            self.cur_file_size -= rem_size;
        }
        use std::io::Write;
        self.file.write(msg).map(|_| ())
    }
}
