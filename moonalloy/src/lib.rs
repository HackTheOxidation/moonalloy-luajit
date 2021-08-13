pub mod linalg;

use crate::linalg::Array;
use crate::linalg::matrix::Matrix;
use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn array_sum(ptr: *mut Array) -> f64 {
    let arr = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    arr.sum()
}

#[no_mangle]
pub extern "C" fn array_print(ptr: *mut Array) {
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
pub extern "C" fn array_add(ptr1: *const Array, ptr2: *const Array) -> *mut Array {
    let arr1 = unsafe {
        assert!(!ptr1.is_null());
        &*ptr1
    };

    let arr2 = unsafe {
        assert!(!ptr2.is_null());
        &*ptr2
    };

    Array::to_raw(arr1.add(arr2))
}

#[no_mangle]
pub extern "C" fn array_sub(ptr1: *const Array, ptr2: *const Array) -> *mut Array {
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
pub extern "C" fn array_mult(ptr1: *const Array, ptr2: *const Array) -> *mut Array {
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
pub extern "C" fn array_dotp(ptr1: *const Array, ptr2: *const Array) -> f64 {
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

#[no_mangle]
pub extern "C" fn array_to_string(ptr: *const Array) -> *const c_char {
    let arr = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    let c_str = CString::new(arr.to_string().as_str()).unwrap();
    let result = c_str.as_ptr();
    std::mem::forget(c_str);
    result
}

#[no_mangle]
pub extern "C" fn array_concat(ptr1: *const Array, ptr2: *const Array) -> *mut Array {
    let arr1 = unsafe {
        assert!(!ptr1.is_null());
        &*ptr1
    };

    let arr2 = unsafe {
        assert!(!ptr2.is_null());
        &*ptr2
    };

    let result = arr1.concat(arr2);

    Array::to_raw(result)
}

#[no_mangle]
pub extern "C" fn array_zeroes(len: i32) -> *mut Array {
    let array = Array::zeroes(len as usize);
    Array::to_raw(array)
}

#[no_mangle]
pub extern "C" fn array_ones(len: i32) -> *mut Array {
    let array = Array::ones(len as usize);
    Array::to_raw(array)
}

#[no_mangle]
pub extern "C" fn matrix_zeroes(rows: i32, cols: i32) -> *mut Matrix {
    let mat = Matrix::zeroes(rows, cols);
    Matrix::to_raw(mat)
}

#[no_mangle]
pub extern "C" fn matrix_ones(rows: i32, cols: i32) -> *mut Matrix {
    let mat = Matrix::ones(rows, cols);
    Matrix::to_raw(mat)
}

#[no_mangle]
pub extern "C" fn matrix_identity(len: i32) -> *mut Matrix {
    let mat = Matrix::identity(len);
    Matrix::to_raw(mat)
}

#[no_mangle]
pub extern "C" fn matrix_print(ptr: *mut Matrix) {
    let mat = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    println!("{}", mat);
}
