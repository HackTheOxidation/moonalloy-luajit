pub mod linalg;
pub mod wrangling;

use crate::linalg::array::Array;
use crate::linalg::matrix::Matrix;
use crate::wrangling::DataTable;
use crate::wrangling::reader::read_csv;
use std::ffi::CString;
use std::os::raw::c_char;
use std::panic::catch_unwind;

// Array
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
pub extern "C" fn array_scalar(ptr: *mut Array, scal: f64) -> *mut Array {
    let arr = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    Array::to_raw(arr.scalar(scal))
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
pub extern "C" fn array_zeros(len: i32) -> *mut Array {
    let array = Array::zeros(len as usize);
    Array::to_raw(array)
}

#[no_mangle]
pub extern "C" fn array_ones(len: i32) -> *mut Array {
    let array = Array::ones(len as usize);
    Array::to_raw(array)
}


// Matrix
#[no_mangle]
pub extern "C" fn matrix_zeros(rows: i32, cols: i32) -> *mut Matrix {
    let mat = Matrix::zeros(rows, cols);
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

#[no_mangle]
pub extern "C" fn matrix_to_string(ptr: *const Matrix) -> *const c_char {
    let mat = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    let c_str = CString::new(mat.to_string().as_str()).unwrap();
    let result = c_str.as_ptr();
    std::mem::forget(c_str);
    result
}

#[no_mangle]
pub extern "C" fn matrix_add(ptr1: *const Matrix, ptr2: *const Matrix) -> *mut Matrix {
    let mat1 = unsafe {
        assert!(!ptr1.is_null());
        &*ptr1
    };

    let mat2 = unsafe {
        assert!(!ptr2.is_null());
        &*ptr2
    };

    Matrix::to_raw(mat1.add(mat2))
}

#[no_mangle]
pub extern "C" fn matrix_scalar(ptr: *const Matrix, scal: f64) -> *mut Matrix {
    let mat = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    Matrix::to_raw(mat.scalar(scal))
}

#[no_mangle]
pub extern "C" fn matrix_sub(ptr1: *const Matrix, ptr2: *const Matrix) -> *mut Matrix {
    let mat1 = unsafe {
        assert!(!ptr1.is_null());
        &*ptr1
    };

    let mat2 = unsafe {
        assert!(!ptr2.is_null());
        &*ptr2
    };

    Matrix::to_raw(mat1.sub(mat2))
}

#[no_mangle]
pub extern "C" fn matrix_elem_mult(ptr1: *const Matrix, ptr2: *const Matrix) -> *mut Matrix {
    let mat1 = unsafe {
        assert!(!ptr1.is_null());
        &*ptr1
    };

    let mat2 = unsafe {
        assert!(!ptr2.is_null());
        &*ptr2
    };

    Matrix::to_raw(mat1.elem_mult(mat2))
}

#[no_mangle]
pub extern "C" fn matrix_transpose(ptr: *const Matrix) -> *mut Matrix {
    let mat = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    Matrix::to_raw(mat.transpose())
}

#[no_mangle]
pub extern "C" fn matrix_mult(ptr1: *const Matrix, ptr2: *const Matrix) -> *mut Matrix {
    let mat1 = unsafe {
        assert!(!ptr1.is_null());
        &*ptr1
    };

    let mat2 = unsafe {
        assert!(!ptr2.is_null());
        &*ptr2
    };

    Matrix::to_raw(mat1.mult(mat2))
}

#[no_mangle]
pub extern "C" fn datatable_read_from_csv(c_str: *mut c_char) -> *const DataTable {
    let path = unsafe { CString::from_raw(c_str) };
    let path_ref = catch_unwind(|| path.to_str().unwrap()).unwrap();
    let dt = read_csv(String::from(path_ref));

    DataTable::to_raw(dt)
}
