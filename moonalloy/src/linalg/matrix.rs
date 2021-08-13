use crate::Array;

use std::alloc::{alloc, Layout};

#[repr(C)]
pub struct Matrix {
    rows: i32,
    cols: i32,
    arrays: *mut Array,
}

impl Matrix {
    pub fn new() -> Matrix {
        let mat_slice = unsafe {
            let layout = Layout::array::<Array>(0).unwrap();
            let ptr = alloc(layout);
            std::slice::from_raw_parts_mut(ptr as *mut Array, 0)
        };

        Matrix {
            rows: 0,
            cols: 0,
            arrays: mat_slice.as_mut_ptr(),
        }
    }
}

