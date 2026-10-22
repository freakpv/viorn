use std::alloc;
use std::ptr;

struct Memory<const N: usize> {
    pos: usize,
    buf: [u8; N],
}

pub struct StackAllocator<const N: usize, A = alloc::System> {
    upstream: A,
    memory: std::sync::Mutex<Memory<N>>,
}

impl<const N: usize, A> StackAllocator<N, A>
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

unsafe impl<const N: usize, A> alloc::Allocator for StackAllocator<N, A> {
    fn allocate(&self, layout: alloc::Layout) -> Result<ptr::NonNull<[u8]>, alloc::AllocError> {
        let mut mem = &mut self.memory.lock().unwrap();
        let mut buf = &mem.buf[mem.pos..];
        let mut ptr = buf.as_mut_ptr();
        let off = ptr.align_offset(layout.align());
        let full_size = off + layout.size();
        if full_size <= buf.len() {
            mem.pos += full_size;
            Ok(unsafe { ptr::NonNull::new_unchecked(ptr.add(off)) })
        } else {
            self.upstream.allocate(layout)
        }
    }
    unsafe fn deallocate(&self, _ptr: ptr::NonNull<u8>, _layout: alloc::Layout) {
        // TODO
    }
}
