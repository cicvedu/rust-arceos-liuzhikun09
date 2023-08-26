//! Simple memory allocation.
//!
//! TODO: more efficient

use core::alloc::Layout;
use core::num::NonZeroUsize;

use crate::{AllocError, AllocResult, BaseAllocator, ByteAllocator};


pub struct SimpleByteAllocator {
    start: usize,
    size: usize,
    next: usize,
    allocations: usize,
}

impl SimpleByteAllocator {
    pub const fn new() -> Self {
        Self {
            start: 0,
            size: 0,
            next: 0,
            allocations: 0,
        }
    }
}

impl BaseAllocator for SimpleByteAllocator {
    fn init(&mut self, start: usize, size: usize) {
        self.start = start;
        self.size = size;
        self.next = start;
        self.allocations = 0;
    }

    fn add_memory(&mut self, start: usize, size: usize) -> AllocResult {
        if self.start == 0 {
            self.init(start, size);
            return Ok(());
        }

        if start == (self.start + self.size) {
            self.size += size;
            Ok(())
        } else {
            Err(AllocError::InvalidParam)
        }
    }
}

impl ByteAllocator for SimpleByteAllocator {
    fn alloc(&mut self, _layout: Layout) -> AllocResult<NonZeroUsize> {
        let size = _layout.size();
        let align = _layout.align();
        let size_with_padding = {
            let padding = (align - (self.next % align)) % align;
            size + padding
        };

        if size_with_padding <= self.next {
            let alloc_addr = self.next;
            //self.used_bytes += size_with_padding;
            self.next += size_with_padding;
            self.allocations += 1;
            Ok(NonZeroUsize::new(alloc_addr).unwrap())
        } else {
            Err(AllocError::NoMemory)
        }
    }

    fn dealloc(&mut self, pos: NonZeroUsize, _layout: Layout) {
        let dealloc_addr = pos.get();
        let size = _layout.size();
        let align = _layout.align();
        let size_with_padding = {
            let padding = (align - (self.next % align)) % align;
            size + padding
        };
        
        // Check if the deallocation address is valid
        if dealloc_addr >= self.start
            && (dealloc_addr + size_with_padding) <= self.next
        {
            // Adjust used_bytes to reflect the deallocation
            let used_bytes = (dealloc_addr - self.start).max(self.next - size_with_padding);
            self.allocations -= 1;
            if self.allocations == 0 {
                self.next = self.start;
            } else {
                self.next = self.size - used_bytes;
            }
        }
    }

    fn total_bytes(&self) -> usize {
        self.size
    }

    fn used_bytes(&self) -> usize {
        self.next
    }

    fn available_bytes(&self) -> usize {
        self.size - self.next
    }
}
