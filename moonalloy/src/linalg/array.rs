use std::alloc::{alloc, Layout};
use std::fmt::*;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone)]
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

    pub fn from(slice: &mut [f64]) -> Array {
        Array {
            len: slice.len() as i32,
            arr: slice.as_mut_ptr(),
        }
    }

    pub fn sum(&self) -> f64 {
        let mut s: f64 = 0.0;
        let v = unsafe { std::slice::from_raw_parts_mut(self.arr, self.len as usize) };

        for i in 0..self.len as usize {
            s += v[i];
        }
        s
    }

    pub fn scalar(&self, scal: f64) -> Array {
        let result = unsafe {
            let layout = Layout::array::<f64>(self.len as usize).unwrap();
            let ptr = alloc(layout) as *mut f64;
            std::slice::from_raw_parts_mut(ptr, self.len as usize)
        };

        let arr_slice = unsafe { std::slice::from_raw_parts_mut(self.arr, self.len as usize) };

        for i in 0..self.len as usize {
            result[i] = scal * arr_slice[i];
        }

        Array {
            arr: result.as_mut_ptr(),
            len: self.len,
        }
    }

    pub fn add(&self, other: &Array) -> Array {
        assert_eq!(self.len, other.len, "Lengths are different!");

        let result = unsafe {
            let layout = Layout::array::<f64>(self.len as usize).unwrap();
            let ptr = alloc(layout) as *mut f64;
            std::slice::from_raw_parts_mut(ptr, self.len as usize)
        };

        let arr1 = unsafe { std::slice::from_raw_parts_mut(self.arr, self.len as usize) };

        let arr2 = unsafe { std::slice::from_raw_parts_mut(other.arr, other.len as usize) };

        for i in 0..self.len as usize {
            result[i] = arr1[i] + arr2[i];
        }

        Array {
            len: result.len() as i32,
            arr: result.as_mut_ptr(),
        }
    }

    pub fn sub(&self, other: &Array) -> Array {
        assert_eq!(self.len, other.len, "Lengths are different!");

        let result = unsafe {
            let layout = Layout::array::<f64>(self.len as usize).unwrap();
            let ptr = alloc(layout) as *mut f64;
            std::slice::from_raw_parts_mut(ptr, self.len as usize)
        };

        let arr1 = unsafe { std::slice::from_raw_parts_mut(self.arr, self.len as usize) };

        let arr2 = unsafe { std::slice::from_raw_parts_mut(other.arr, other.len as usize) };

        for i in 0..self.len as usize {
            result[i] = arr1[i] - arr2[i];
        }

        Array {
            len: result.len() as i32,
            arr: result.as_mut_ptr(),
        }
    }

    pub fn mult(&self, other: &Array) -> Array {
        assert_eq!(self.len, other.len, "Lengths are different!");

        let result = unsafe {
            let layout = Layout::array::<f64>(self.len as usize).unwrap();
            let ptr = alloc(layout) as *mut f64;
            std::slice::from_raw_parts_mut(ptr, self.len as usize)
        };

        let arr1 = unsafe { std::slice::from_raw_parts_mut(self.arr, self.len as usize) };

        let arr2 = unsafe { std::slice::from_raw_parts_mut(other.arr, other.len as usize) };

        for i in 0..self.len as usize {
            result[i] = arr1[i] * arr2[i];
        }

        Array {
            len: result.len() as i32,
            arr: result.as_mut_ptr(),
        }
    }

    pub fn dotp(&self, other: &Array) -> f64 {
        let arr = self.mult(other);
        let v = unsafe { std::slice::from_raw_parts_mut(arr.arr, arr.len as usize) };
        v.iter().sum()
    }

    pub fn concat(&self, other: &Array) -> Array {
        let len = (self.len + other.len) as usize;
        let result = unsafe {
            let layout = Layout::array::<f64>(len).unwrap();
            let ptr = alloc(layout) as *mut f64;
            std::slice::from_raw_parts_mut(ptr, len)
        };

        let arr1 = unsafe { std::slice::from_raw_parts_mut(self.arr, self.len as usize) };

        let arr2 = unsafe { std::slice::from_raw_parts_mut(other.arr, other.len as usize) };

        let mut i = 0;
        for elem in arr1.iter() {
            result[i] = *elem;
            i += 1;
        }

        for elem in arr2.iter() {
            result[i] = *elem;
            i += 1;
        }

        Array {
            len: result.len() as i32,
            arr: result.as_mut_ptr(),
        }
    }

    pub fn to_string(&self) -> String {
        let array_slice = unsafe { std::slice::from_raw_parts_mut(self.arr, self.len as usize) };

        format!("Array: {:?}", array_slice)
    }

    pub fn to_raw(arr: Array) -> *mut Array {
        Box::into_raw(Box::new(arr))
    }

    pub fn of(val: f64, len: usize) -> Array {
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
        assert!(
            index < self.len as usize,
            "ERROR - Array get: Index out of bounds."
        );
        let slice = unsafe { std::slice::from_raw_parts_mut(self.arr, self.len as usize) };

        slice[index]
    }

    pub fn set(&self, val: f64, index: usize) {
        assert!(
            index < self.len as usize,
            "ERROR - Array get: Index out of bounds."
        );
        let slice = unsafe { std::slice::from_raw_parts_mut(self.arr, self.len as usize) };

        slice[index] = val;
    }
}

impl std::fmt::Display for Array {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.to_string())
    }
}

impl PartialEq for Array {
    fn eq(&self, other: &Self) -> bool {
        if self.len != other.len {
            return false;
        }

        let slice1 = unsafe { std::slice::from_raw_parts_mut(self.arr, self.len as usize) };

        let slice2 = unsafe { std::slice::from_raw_parts_mut(other.arr, other.len as usize) };

        for i in 0..self.len as usize {
            if slice1[i] != slice2[i] {
                return false;
            }
        }

        true
    }
}

impl Deref for Array {
    type Target = [f64];
    fn deref(&self) -> &[f64] {
        unsafe { std::slice::from_raw_parts(self.arr, self.len as usize) }
    }
}

impl DerefMut for Array {
    fn deref_mut(&mut self) -> &mut [f64] {
        unsafe { std::slice::from_raw_parts_mut(self.arr, self.len as usize) }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let n = Array::new();
        let f = Array::from(&mut []);

        assert_eq!(n, f);
    }

    #[test]
    fn sum() {
        let a = Array::from(&mut [1.0, 2.0, 3.0]);

        assert_eq!(6.0, a.sum());
    }

    #[test]
    fn scalar() {
        let a = Array::from(&mut [1.0, 2.0, 3.0]);
        let r = Array::from(&mut [2.0, 4.0, 6.0]);

        assert_eq!(r, a.scalar(2.0))
    }

    #[test]
    fn add() {
        let a = Array::from(&mut [1.0, 2.0, 3.0]);
        let b = Array::from(&mut [2.0, 3.0, 5.0]);
        let r = Array::from(&mut [3.0, 5.0, 8.0]);

        assert_eq!(r, a.add(&b));
    }

    #[test]
    fn sub() {
        let a = Array::from(&mut [2.0, 3.0, 5.0]);
        let b = Array::from(&mut [1.0, 2.0, 3.0]);
        let r = Array::from(&mut [1.0, 1.0, 2.0]);

        assert_eq!(r, a.sub(&b));
    }

    #[test]
    fn mult() {
        let a = Array::from(&mut [1.0, 2.0, 3.0]);
        let b = Array::from(&mut [2.0, 3.0, 5.0]);
        let r = Array::from(&mut [2.0, 6.0, 15.0]);

        assert_eq!(r, a.mult(&b));
    }

    #[test]
    fn dotp() {
        let a = Array::from(&mut [1.0, 2.0, 3.0]);
        let b = Array::from(&mut [2.0, 3.0, 5.0]);

        assert_eq!(23.0, a.dotp(&b));
    }

    #[test]
    fn concat() {
        let a = Array::from(&mut [1.0, 2.0]);
        let b = Array::from(&mut [3.0, 5.0]);
        let r = Array::from(&mut [1.0, 2.0, 3.0, 5.0]);

        assert_eq!(r, a.concat(&b));
    }

    #[test]
    fn zeros() {
        let a = Array::zeros(3);
        let r = Array::from(&mut [0.0, 0.0, 0.0]);

        assert_eq!(r, a);
    }

    #[test]
    fn ones() {
        let a = Array::ones(3);
        let r = Array::from(&mut [1.0, 1.0, 1.0]);

        assert_eq!(r, a);
    }

    #[test]
    fn get() {
        let a = Array::from(&mut [1.0, 2.0, 3.0]);

        assert_eq!(2.0, a.get(1));
    }

    #[test]
    fn set() {
        let a = Array::from(&mut [1.0, 2.0, 3.0]);
        let r = Array::from(&mut [5.0, 2.0, 3.0]);

        a.set(5.0, 0);

        assert_eq!(r, a);
    }

    #[test]
    fn iterator() {
        let a = Array::from(&mut [1.0, 2.0, 3.0]);
        let mut it = a.iter();

        assert_eq!(*it.next().unwrap(), 1.0 as f64);
        assert_eq!(*it.next().unwrap(), 2.0 as f64);
        assert_eq!(*it.next().unwrap(), 3.0 as f64);
    }
}
