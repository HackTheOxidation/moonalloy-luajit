use crate::Array;

use std::fmt::*;
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

    fn of(val: f64, rows: i32, cols: i32) -> Matrix {
        let mat_slice = unsafe {
            let layout = Layout::array::<Array>(cols as usize).unwrap();
            let ptr = alloc(layout);
            std::slice::from_raw_parts_mut(ptr as *mut Array, cols as usize)
        };

        for i in 0..cols as usize {
            mat_slice[i] = Array::of(val, rows as usize);
        }

        Matrix {
            rows,
            cols,
            arrays: mat_slice.as_mut_ptr(),
        }
    }

    pub fn zeros(rows: i32, cols: i32) -> Matrix {
        Matrix::of(0.0, rows, cols)
    }

    pub fn ones(rows: i32, cols: i32) -> Matrix {
        Matrix::of(1.0, rows, cols)
    }

    pub fn identity(len: i32) -> Matrix {
        let mat = Matrix::zeros(len, len);

        let mat_slice = unsafe {
            std::slice::from_raw_parts_mut(mat.arrays, len as usize)
        };

        for i in 0..len as usize {
            let slice = unsafe {
                std::slice::from_raw_parts_mut(mat_slice[i].arr, len as usize)
            };

            slice[i] = 1.0;
        }

        mat
    }

    pub fn to_string(&self) -> String {
        let array_slice = unsafe {
            std::slice::from_raw_parts_mut(self.arrays, self.cols as usize)
        };

        let mut result = String::from("Matrix: [");

        for (i, arr) in array_slice.iter().enumerate() {
            let slice = unsafe {
                std::slice::from_raw_parts_mut(arr.arr, arr.len as usize)
            };

            result.push_str(format!("{:?}", slice).as_str());
            
            if i < (arr.len - 1) as usize {
                result.push_str(", ")
            }
        }

        result.push(']');

        result
    }

    pub fn to_raw(mat: Matrix) -> *mut Matrix {
        Box::into_raw(Box::new(mat))
    }
}

impl Display for Matrix {

    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f,"{}", self.to_string())
    }
}  

