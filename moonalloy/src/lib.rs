use std::fmt::*;

#[repr(C)]
pub struct Array {
    pub len: i32,
    pub arr: Vec<f64>,
}

impl Array {
    pub fn new() -> Array {
        Array {
            len: 0,
            arr: Vec::new(),
        }
    }

    pub fn from(len: i32, arr: Vec<f64>) -> Array {
        Array { len, arr }
    }
    
    pub fn sum(&self) -> f64 {
        let mut s: f64 = 0.0;
        for i in 0..self.len as usize {
            s += self.arr[i];
        }
        s
    }

    pub fn add(&self, other: &Array) -> Array {
        if self.len != other.len {
            panic!("Lengths are different!");
        }

        let mut vector = Vec::new();

        for i in 0..self.len as usize {
            vector.push(self.arr[i] + other.arr[i]);
        }

        Array::from(vector.len() as i32, vector)
    }

    pub fn sub(&self, other: &Array) -> Array {
        if self.len != other.len {
            panic!("Lengths are different!");
        }

        let mut vector = Vec::new();

        for i in 0..self.len as usize {
            vector.push(self.arr[i] - other.arr[i]);
        }

        Array::from(vector.len() as i32, vector)
    }

    pub fn mult(&self, other: &Array) -> Array {
        if self.len != other.len {
            panic!("Lengths are different!");
        }

        let mut vector = Vec::new();

        for i in 0..self.len as usize {
            vector.push(self.arr[i] * other.arr[i]);
        }

        Array::from(vector.len() as i32, vector)
    }

    pub fn dotp(&self, other: &Array) -> f64 {
        let arr = self.add(other);
        arr.arr.iter().sum()
    }

    pub fn to_raw(arr: Array) -> *mut Array {
        Box::into_raw(Box::new(arr))
    }
}

impl std::fmt::Display for Array {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut temp = Vec::new();
        for num in 0..self.len as usize {
            temp.push(self.arr[num]);
        }
        write!(f, "Array: len = {}, arr = {:?}", self.len, temp) 
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

    let result = arr1.add(arr2);

    Array::to_raw(result)
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
