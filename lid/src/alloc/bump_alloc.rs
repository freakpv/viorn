use std::alloc;
use std::ptr;

struct Buffer<'a> {
    pos: usize,
    buf: &'a mut [u8],
}

pub struct BumpAllocator<'a, A = alloc::System> {
    upstream: A,
    // TODO: Replace
    buffer: std::sync::Mutex<Buffer<'a>>,
}

impl<'a, A> BumpAllocator<'a, A>
where
    A: alloc::Allocator,
{
    pub fn with_upstream(buf: &'a mut [u8], upstream: A) -> Self {
        Self {
            upstream: upstream,
            buffer: Buffer { pos: 0, buf: buf }.into(),
        }
    }
}

unsafe impl<'a, A: alloc::Allocator> alloc::Allocator for BumpAllocator<'a, A> {
    fn allocate(&self, layout: alloc::Layout) -> Result<ptr::NonNull<[u8]>, alloc::AllocError> {
        let mut mem = self.buffer.lock().unwrap();
        let pos = mem.pos;
        let buf = &mut mem.buf[pos..];
        let base = buf.as_mut_ptr();
        let offset = base.align_offset(layout.align());
        let size = offset.checked_add(layout.size()).ok_or(alloc::AllocError)?;
        if size <= buf.len() {
            mem.pos += size;
            Ok(unsafe {
                ptr::NonNull::slice_from_raw_parts(
                    ptr::NonNull::new_unchecked(base.add(offset)),
                    layout.size(),
                )
            })
        } else {
            // Don't hold the mutex lock while calling the upstream allocator
            drop(mem);
            self.upstream.allocate(layout)
        }
    }
    unsafe fn deallocate(&self, ptr: ptr::NonNull<u8>, layout: alloc::Layout) {
        // Don't hold the mutex lock while calling the upstream allocator
        let is_internal = self
            .buffer
            .lock()
            .unwrap()
            .buf
            .as_ptr_range()
            .contains(&ptr.as_ptr().cast_const());
        if !is_internal {
            unsafe { self.upstream.deallocate(ptr, layout) };
        }
    }
}
