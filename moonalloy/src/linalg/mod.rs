use std::fmt::*;
use std::mem;
use std::alloc::{alloc, Layout};

pub mod matrix;

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

    pub fn scalar(&self, scal: f64) -> Array {
        let result = unsafe {
            let layout = Layout::array::<f64>(self.len as usize).unwrap();
            let ptr = alloc(layout) as *mut f64;
            std::slice::from_raw_parts_mut(ptr, self.len as usize)
        };

        let arr_slice = unsafe {
            std::slice::from_raw_parts_mut(self.arr, self.len as usize)
        };

        for i in 0..self.len as usize {
            result[i] = scal * arr_slice[i];
        }

        Array {
            arr: result.as_mut_ptr(),
            len: self.len,
        }
    }

    pub fn add(&self, other: &Array) -> Array {
        if self.len != other.len {
            panic!("Lengths are different!");
        }

        let result = unsafe {
            let layout = Layout::array::<f64>(self.len as usize).unwrap();
            let ptr = alloc(layout) as *mut f64;
            std::slice::from_raw_parts_mut(ptr, self.len as usize)
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
            let layout = Layout::array::<f64>(self.len as usize).unwrap();
            let ptr = alloc(layout) as *mut f64;
            std::slice::from_raw_parts_mut(ptr, self.len as usize)
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
            let layout = Layout::array::<f64>(self.len as usize).unwrap();
            let ptr = alloc(layout) as *mut f64;
            std::slice::from_raw_parts_mut(ptr, self.len as usize)
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

    pub fn concat(&self, other: &Array) -> Array {
        let len = (self.len + other.len) as usize;
        let result = unsafe {
            let layout = Layout::array::<f64>(len).unwrap();
            let ptr = alloc(layout) as *mut f64;
            std::slice::from_raw_parts_mut(ptr, len)
        };

        let arr1 = unsafe {
            vec_from_raw(self.arr, self.len as usize)
        };

        let arr2 = unsafe {
            vec_from_raw(other.arr, other.len as usize)
        };

        let mut i = 0;
        for elem in arr1.iter() {
            result[i] = *elem;
            i += 1;
        }

        for elem in arr2.iter() {
            result[i] = *elem;
            i += 1;
        }

        mem::forget(arr1);
        mem::forget(arr2);

        Array { len: result.len() as i32, arr: result.as_mut_ptr() }
    }

    pub fn to_string(&self) -> String {
        let array_slice = unsafe {
            std::slice::from_raw_parts_mut(self.arr, self.len as usize)
        };
        
        format!("Array: {:?}", array_slice)
    }

    pub fn to_raw(arr: Array) -> *mut Array {
        Box::into_raw(Box::new(arr))
    }

    fn of(val: f64, len: usize) -> Array {
        let arr_slice = unsafe {
            let layout = Layout::array::<f64>(len).unwrap();
            let ptr = alloc(layout);
            std::slice::from_raw_parts_mut(ptr as *mut f64, len)
        };

        for i in 0..len {
            arr_slice[i] = val;
        }

        Array {
            arr: arr_slice.as_mut_ptr(),
            len: len as i32,
        }
    }

    pub fn zeros(len: usize) -> Array {
        Array::of(0.0, len)
    }

    pub fn ones(len: usize) -> Array {
        Array::of(1.0, len)
    }

    pub fn get(&self, index: usize) -> f64 {
        assert!(index < self.len as usize, "ERROR - Array get: Index out of bounds.");
        let slice = unsafe {
            std::slice::from_raw_parts_mut(self.arr, self.len as usize)
        };

        slice[index]
    }

    pub fn set(&self, val: f64, index: usize) {
        assert!(index < self.len as usize, "ERROR - Array get: Index out of bounds.");
        let slice = unsafe {
            std::slice::from_raw_parts_mut(self.arr, self.len as usize)
        };

        slice[index] = val;
    }
}

impl std::fmt::Display for Array {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.to_string())
    }
}

unsafe fn vec_from_raw(arr: *mut f64, len: usize) -> Vec<f64> {
    let mut result = Vec::from_raw_parts(arr, len, len);
    result.resize(len, 0.0);
    result
}
