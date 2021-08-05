use std::fmt::*;
use std::mem;
use std::alloc::{alloc, Layout};

#[repr(C)]
pub struct Array {
    pub len: i32,
    pub arr: *mut f64,
}

impl Array {
    pub fn new() -> Array {
        let arr_slice = unsafe {
            let layout = Layout::new::<f64>();
            let ptr = alloc(layout);
            std::slice::from_raw_parts_mut(ptr as *mut f64, 0)
        };

        Array {
            len: 0,
            arr: arr_slice.as_mut_ptr(),
        }
    }
    
    pub fn sum(&self) -> f64 {
        let mut s: f64 = 0.0;
        let v = unsafe {
            vec_from_raw(self.arr, self.len as usize)
        };

        for i in 0..self.len as usize {
            s += v[i];
        }
        mem::forget(v);
        s
    }

    pub fn add(&self, other: &Array) -> Array {
        if self.len != other.len {
            panic!("Lengths are different!");
        }

        let result = unsafe {
            let layout = Layout::new::<f64>();
            let ptr = alloc(layout);
            std::slice::from_raw_parts_mut(ptr as *mut f64, self.len as usize)
        };

        let arr1 = unsafe {
            vec_from_raw(self.arr, self.len as usize)
        };

        let arr2 = unsafe {
            vec_from_raw(other.arr, other.len as usize)
        };

        for i in 0..self.len as usize {
            result[i] = arr1[i] + arr2[i];
        }

        mem::forget(arr1);
        mem::forget(arr2);

        Array { len: result.len() as i32, arr: result.as_mut_ptr() }
    }

    pub fn sub(&self, other: &Array) -> Array {
        if self.len != other.len {
            panic!("Lengths are different!");
        }

        let result = unsafe {
            let layout = Layout::new::<f64>();
            let ptr = alloc(layout);
            std::slice::from_raw_parts_mut(ptr as *mut f64, self.len as usize)
        };

        let arr1 = unsafe {
            vec_from_raw(self.arr, self.len as usize)
        };

        let arr2 = unsafe {
            vec_from_raw(other.arr, other.len as usize)
        };

        for i in 0..self.len as usize {
            result[i] = arr1[i] - arr2[i];
        }

        mem::forget(arr1);
        mem::forget(arr2);

        Array { len: result.len() as i32, arr: result.as_mut_ptr() }
    }

    pub fn mult(&self, other: &Array) -> Array {
        if self.len != other.len {
            panic!("Lengths are different!");
        }

        let result = unsafe {
            let layout = Layout::new::<f64>();
            let ptr = alloc(layout);
            std::slice::from_raw_parts_mut(ptr as *mut f64, self.len as usize)
        };

        let arr1 = unsafe {
            vec_from_raw(self.arr, self.len as usize)
        };

        let arr2 = unsafe {
            vec_from_raw(other.arr, other.len as usize)
        };

        for i in 0..self.len as usize {
            result[i] = arr1[i] * arr2[i];
        }

        mem::forget(arr1);
        mem::forget(arr2);

        Array { len: result.len() as i32, arr: result.as_mut_ptr() }
    }

    pub fn dotp(&self, other: &Array) -> f64 {
        let arr = self.add(other);
        let v = unsafe {
            vec_from_raw(arr.arr, arr.len as usize)
        };
        mem::forget(arr);
        v.iter().sum()
    }

    pub fn to_raw(arr: Array) -> *mut Array {
        Box::into_raw(Box::new(arr))
    }

}

impl std::fmt::Display for Array {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let temp = unsafe {
            vec_from_raw(self.arr, self.len as usize)
        };

        let result = write!(f, "Array: len = {}, arr = {:?}", self.len, temp);
        mem::forget(temp);
        result
    }
}

unsafe fn vec_from_raw(arr: *mut f64, len: usize) -> Vec<f64> {
    let mut result = Vec::from_raw_parts(arr, len, len);
    result.resize(len, 0.0);
    result
}