use std::fmt::*;
use std::mem;

#[repr(C)]
pub struct Array {
    pub len: i32,
    pub arr: *mut f64,
}

impl Array {
    pub fn new() -> Array {
        let mut v: Vec<f64> = Vec::with_capacity(0);
        Array {
            len: 0,
            arr: v.as_mut_ptr(),
        }
    }

    pub fn from(len: i32, mut arr: Vec<f64>) -> Array {
        let result = Array { len, arr: arr.as_mut_ptr() };
        println!("In Rust - Result = {}", result);
        result
    }
    
    pub fn sum(&self) -> f64 {
        let mut s: f64 = 0.0;
        let v = unsafe {
            vec_from_raw(self.arr, self.len as usize)
        };

        for i in 0..self.len as usize {
            s += v[i];
        }
        s
    }

    pub fn add(&self, other: &Array) -> Array {
        if self.len != other.len {
            panic!("Lengths are different!");
        }

        let mut result = Vec::new();

        let arr1 = unsafe {
            vec_from_raw(self.arr, self.len as usize)
        };

        let arr2 = unsafe {
            vec_from_raw(other.arr, other.len as usize)
        };

        for i in 0..self.len as usize {
            result.push(arr1[i] + arr2[i]);
        }

        mem::forget(arr1);
        mem::forget(arr2);

        Array::from(result.len() as i32, result)
    }

    pub fn sub(&self, other: &Array) -> Array {
        if self.len != other.len {
            panic!("Lengths are different!");
        }

        let mut result = Vec::new();

        let arr1 = unsafe {
            vec_from_raw(self.arr, self.len as usize)
        };

        let arr2 = unsafe {
            vec_from_raw(other.arr, other.len as usize)
        };

        for i in 0..self.len as usize {
            result.push(arr1[i] - arr2[i]);
        }

        mem::forget(arr1);
        mem::forget(arr2);

        Array::from(result.len() as i32, result)
    }

    pub fn mult(&self, other: &Array) -> Array {
        if self.len != other.len {
            panic!("Lengths are different!");
        }

        let mut result = Vec::new();

        let arr1 = unsafe {
            vec_from_raw(self.arr, self.len as usize)
        };

        let arr2 = unsafe {
            vec_from_raw(other.arr, other.len as usize)
        };

        for i in 0..self.len as usize {
            result.push(arr1[i] * arr2[i]);
        }

        mem::forget(arr1);
        mem::forget(arr2);

        Array::from(result.len() as i32, result)
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

unsafe fn vec_from_raw(arr: *mut f64, len: usize) -> Vec<f64> {
    let mut result = Vec::from_raw_parts(arr, len, len);
    result.resize(len, 0.0);
    result
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

#[no_mangle]
pub extern "C" fn sum(ptr: *mut Array) -> f64 {
    let arr = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    arr.sum()
}

#[no_mangle]
pub extern "C" fn print(ptr: *mut Array) {
    let arr = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    println!("{}", arr);
}

#[no_mangle]
pub extern "C" fn array_new() -> *mut Array {
    Box::into_raw(Box::new(Array::new()))
}

#[no_mangle]
pub extern "C" fn add(ptr1: *const Array, ptr2: *const Array) -> *mut Array {
    let arr1 = unsafe {
        assert!(!ptr1.is_null());
        &*ptr1
    };

    let arr2 = unsafe {
        assert!(!ptr2.is_null());
        &*ptr2
    };

    Array::to_raw(arr1.add(arr2))
    /*
    let result = arr1.add(arr2);
    let mut boxed = Box::new(result);

    &mut *boxed
    */
}

#[no_mangle]
pub extern "C" fn sub(ptr1: *const Array, ptr2: *const Array) -> *mut Array {
    let arr1 = unsafe {
        assert!(!ptr1.is_null());
        &*ptr1
    };

    let arr2 = unsafe {
        assert!(!ptr2.is_null());
        &*ptr2
    };

    let result = arr1.sub(arr2);

    Array::to_raw(result)
}

#[no_mangle]
pub extern "C" fn mult(ptr1: *const Array, ptr2: *const Array) -> *mut Array {
    let arr1 = unsafe {
        assert!(!ptr1.is_null());
        &*ptr1
    };

    let arr2 = unsafe {
        assert!(!ptr2.is_null());
        &*ptr2
    };

    let result = arr1.mult(arr2);

    Array::to_raw(result)
}

#[no_mangle]
pub extern "C" fn dotp(ptr1: *const Array, ptr2: *const Array) -> f64 {
    let arr1 = unsafe {
        assert!(!ptr1.is_null());
        &*ptr1
    };

    let arr2 = unsafe {
        assert!(!ptr2.is_null());
        &*ptr2
    };

    arr1.dotp(arr2)
}
