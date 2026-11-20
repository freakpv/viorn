use std::alloc;
use std::ptr;

struct Memory<const N: usize> {
    pos: usize,
    buf: [u8; N],
}

pub struct BumpAllocator<const N: usize, A = alloc::System> {
    upstream: A,
    // TODO: Replace
    memory: std::sync::Mutex<Memory<N>>,
}

impl<const N: usize, A> BumpAllocator<N, A>
where
    A: alloc::Allocator,
{
    pub fn with_upstream(upstream: A) -> Self {
        Self {
            upstream: upstream,
            memory: Memory::<N> {
                pos: 0,
                buf: [0; N],
            }
            .into(),
        }
    }
}

unsafe impl<const N: usize, A: alloc::Allocator> alloc::Allocator for BumpAllocator<N, A> {
    fn allocate(&self, layout: alloc::Layout) -> Result<ptr::NonNull<[u8]>, alloc::AllocError> {
        let mut mem = self.memory.lock().unwrap();
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
            .memory
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
