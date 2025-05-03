//! Global Allocator for UEFI.
//! Uses [r-efi-alloc](https://crates.io/crates/r-efi-alloc)

use crate::alloc::{GlobalAlloc, Layout, System};
use crate::sys::pal::helpers;

unsafe extern "C" {
    fn malloc(size: usize) -> *mut core::ffi::c_void;
    fn free(ptr: *mut core::fi::c_void);
}

#[stable(feature = "alloc_system_type", since = "1.28.0")]
unsafe impl GlobalAlloc for System {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        return unsafe { malloc(layout.size()) } as *mut u8;
        // todo: align malloc
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        return unsafe { free(ptr as *mut core::ffi::c_void) };
    }
}
