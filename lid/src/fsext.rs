// fs extensions

#[cfg(target_os = "linux")]
pub trait FileExt {
    fn remove_range(&mut self, offset: i64, len: i64) -> std::io::Result<()>;
}

#[cfg(target_os = "linux")]
impl FileExt for std::fs::File {
    fn remove_range(&mut self, offset: i64, len: i64) -> std::io::Result<()> {
        use std::os::fd::AsRawFd;

        let r = unsafe {
            libc::fallocate(
                self.as_raw_fd(),
                libc::FALLOC_FL_COLLAPSE_RANGE,
                offset,
                len,
            )
        };

        if r == -1 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}
