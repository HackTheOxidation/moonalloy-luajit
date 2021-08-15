use crate::Array;

use std::fmt::*;
use std::alloc::{alloc, Layout};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Matrix {
    rows: i32,
    cols: i32,
    arrays: *mut Array,
}

impl Matrix {
    fn is_valid_slice(slice: &mut [Array]) -> bool {
        let len = slice[0].len;
        for i in 1..slice.len() {
            assert!(len == slice[i].len);
        }

        true
    }

    pub fn new(slice: &mut [Array]) -> Matrix {
        assert!(Matrix::is_valid_slice(slice));
        Matrix {
            rows: slice.len() as i32,
            cols: slice[0].len,
            arrays: slice.as_mut_ptr(),
        }
    }

    fn of(val: f64, rows: i32, cols: i32) -> Matrix {
        let mat_slice = unsafe {
            let layout = Layout::array::<Array>(rows as usize).unwrap();
            let ptr = alloc(layout);
            std::slice::from_raw_parts_mut(ptr as *mut Array, rows as usize)
        };

        for i in 0..rows as usize {
            mat_slice[i] = Array::of(val, cols as usize);
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
            std::slice::from_raw_parts_mut(self.arrays, self.rows as usize)
        };

        let mut result = String::from("Matrix: \n[");

        for (i, arr) in array_slice.iter().enumerate() {
            let slice = unsafe {
                std::slice::from_raw_parts_mut(arr.arr, arr.len as usize)
            };

            result.push_str(format!("{:?}", slice).as_str());
            
            if i < (arr.len - 1) as usize {
                result.push_str(", \n ")
            }
        }

        result.push(']');

        result
    }

    pub fn add(&self, other: &Matrix) -> Matrix {
        assert!(self.cols == other.cols, "ERROR - Matrix addition: Columns differ in dimensions.");
        assert!(self.rows == other.rows, "ERROR - Matrix addition: Rows differ in dimensions.");

        let result = unsafe {
            let layout = Layout::array::<Array>(self.rows as usize).unwrap();
            let ptr = alloc(layout);
            std::slice::from_raw_parts_mut(ptr as *mut Array, self.rows as usize)
        };

        let mat_slice1 = unsafe {
            std::slice::from_raw_parts_mut(self.arrays, self.rows as usize)
        };

        let mat_slice2 = unsafe {
            std::slice::from_raw_parts_mut(other.arrays, other.rows as usize)
        };

        for i in 0..self.rows as usize {
            result[i] = mat_slice1[i].add(&mat_slice2[i]);
        }

        Matrix {
            rows: self.rows,
            cols: self.cols,
            arrays: result.as_mut_ptr(),
        }
    }

    pub fn scalar(&self, scal: f64) -> Matrix {
        let result = unsafe {
            let layout = Layout::array::<Array>(self.rows as usize).unwrap();
            let ptr = alloc(layout);
            std::slice::from_raw_parts_mut(ptr as *mut Array, self.rows as usize)
        };

        let slice = unsafe {
            std::slice::from_raw_parts_mut(self.arrays, self.rows as usize)
        };

        for i in 0..self.rows as usize {
            result[i] = slice[i].scalar(scal);
        }

        Matrix {
            rows: self.rows,
            cols: self.cols,
            arrays: result.as_mut_ptr(),
        }
    }

    pub fn sub(&self, other: &Matrix) -> Matrix {
        assert!(self.cols == other.cols, "ERROR - Matrix subtraction: Columns differ in dimensions.");
        assert!(self.rows == other.rows, "ERROR - Matrix subtraction: Rows differ in dimensions.");

        let result = unsafe {
            let layout = Layout::array::<Array>(self.rows as usize).unwrap();
            let ptr = alloc(layout);
            std::slice::from_raw_parts_mut(ptr as *mut Array, self.rows as usize)
        };

        let mat_slice1 = unsafe {
            std::slice::from_raw_parts_mut(self.arrays, self.rows as usize)
        };

        let mat_slice2 = unsafe {
            std::slice::from_raw_parts_mut(other.arrays, other.rows as usize)
        };

        for i in 0..self.rows as usize {
            result[i] = mat_slice1[i].sub(&mat_slice2[i]);
        }

        Matrix {
            rows: self.rows,
            cols: self.cols,
            arrays: result.as_mut_ptr(),
        }
    }

    pub fn elem_mult(&self, other: &Matrix) -> Matrix {
        assert!(self.cols == other.cols, "ERROR - Matrix element-wise multiplication: Columns differ in dimensions.");
        assert!(self.rows == other.rows, "ERROR - Matrix element-wise multiplication: Rows differ in dimensions.");

        let result = unsafe {
            let layout = Layout::array::<Array>(self.rows as usize).unwrap();
            let ptr = alloc(layout);
            std::slice::from_raw_parts_mut(ptr as *mut Array, self.rows as usize)
        };

        let mat_slice1 = unsafe {
            std::slice::from_raw_parts_mut(self.arrays, self.rows as usize)
        };

        let mat_slice2 = unsafe {
            std::slice::from_raw_parts_mut(other.arrays, other.rows as usize)
        };

        for i in 0..self.rows as usize {
            result[i] = mat_slice1[i].mult(&mat_slice2[i]);
        }

        Matrix {
            rows: self.rows,
            cols: self.cols,
            arrays: result.as_mut_ptr(),
        }
    }

    pub fn transpose(&self) -> Matrix {
        let result = unsafe {
            let layout = Layout::array::<Array>(self.cols as usize).unwrap();
            let ptr = alloc(layout);
            std::slice::from_raw_parts_mut(ptr as *mut Array, self.cols as usize)
        };

        let arr_slice = unsafe {
            std::slice::from_raw_parts_mut(self.arrays, self.rows as usize)
        };

        for i in 0..self.cols as usize {
            result[i] = Array::zeros(self.rows as usize);

            for j in 0..self.rows as usize {
                result[i].set(arr_slice[j].get(i), j);
            }
        }

        Matrix {
            rows: self.cols,
            cols: self.rows,
            arrays: result.as_mut_ptr(),
        }
    }

    pub fn mult(&self, other: &Matrix) -> Matrix {
        assert!(self.rows == other.cols, "ERROR - Matrix multiplication: Invalid dimensions.");

        let result = unsafe {
            let layout = Layout::array::<Array>(self.cols as usize).unwrap();
            let ptr = alloc(layout);
            std::slice::from_raw_parts_mut(ptr as *mut Array, self.cols as usize)
        };

        let mat_t = self.transpose();

        let mat_slice1 = unsafe {
            std::slice::from_raw_parts_mut(mat_t.arrays, mat_t.rows as usize)
        };

        let mat_slice2 = unsafe {
            std::slice::from_raw_parts_mut(other.arrays, other.rows as usize)
        };

        for i in 0..self.cols as usize {
            result[i] = Array::zeros(other.rows as usize);

            for j in 0..other.rows as usize {
                result[i].set(mat_slice1[j].dotp(&mat_slice2[i]), j);
            }
        }

        Matrix {
            rows: other.rows,
            cols: self.cols,
            arrays: result.as_mut_ptr(),
        }
    }

    pub fn get(&self, i: usize, j: usize) -> f64 {
        let slice = unsafe {
            std::slice::from_raw_parts_mut(self.arrays, self.rows as usize)
        };

        slice[i].get(j)
    }

    pub fn set(&self, val: f64, i: usize, j: usize) {
        let slice = unsafe {
            std::slice::from_raw_parts_mut(self.arrays, self.rows as usize)
        };

        slice[i].set(val, j);
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

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.rows != other.rows {
            return false;
        }

        if self.cols != other.cols {
            return false;
        }

        let slice1 = unsafe {
            std::slice::from_raw_parts_mut(self.arrays, self.rows as usize)
        };

        let slice2 = unsafe {
            std::slice::from_raw_parts_mut(other.arrays, other.rows as usize)
        };

        for i in 0..self.rows as usize {
            if slice1[i] != slice2[i] {
                return false;
            }
        }

        true
    }
}

impl Deref for Matrix {
    type Target = [Array];
    fn deref(&self) -> &[Array] {
        unsafe {
            std::slice::from_raw_parts(self.arrays, self.rows as usize)
        }
    }
}

impl DerefMut for Matrix {
    fn deref_mut(&mut self) -> &mut [Array] {
        unsafe {
            std::slice::from_raw_parts_mut(self.arrays, self.rows as usize)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn zeros() {
        let z = Matrix::zeros(2, 2);
        let r = Matrix::new(&mut [Array::from(&mut [0.0, 0.0]), Array::from(&mut [0.0, 0.0])]);

        assert_eq!(r, z);
    }

    #[test]
    fn ones() {
        let o = Matrix::ones(2, 2);
        let r = Matrix::new(&mut [Array::from(&mut [1.0, 1.0]), Array::from(&mut [1.0, 1.0])]);

        assert_eq!(r, o);
    }

    #[test]
    fn identity() {
        let i = Matrix::identity(2);
        let r = Matrix::new(&mut [Array::from(&mut [1.0, 0.0]), Array::from(&mut [0.0, 1.0])]);

        assert_eq!(r, i);
    }

    #[test]
    fn add() {
        let a = Matrix::new(&mut [Array::from(&mut [1.0, 2.0]), Array::from(&mut [3.0, 5.0])]);
        let b = Matrix::new(&mut [Array::from(&mut [2.0, 3.0]), Array::from(&mut [5.0, 8.0])]);
        let r = Matrix::new(&mut [Array::from(&mut [3.0, 5.0]), Array::from(&mut [8.0, 13.0])]);
        
        assert_eq!(r, a.add(&b));
    }

    #[test]
    fn sub() {
        let a = Matrix::new(&mut [Array::from(&mut [3.0, 5.0]), Array::from(&mut [8.0, 13.0])]);
        let b = Matrix::new(&mut [Array::from(&mut [2.0, 3.0]), Array::from(&mut [5.0, 8.0])]);
        let r = Matrix::new(&mut [Array::from(&mut [1.0, 2.0]), Array::from(&mut [3.0, 5.0])]);
        
        assert_eq!(r, a.sub(&b));
    }

    #[test]
    fn scalar() {
        let a = Matrix::new(&mut [Array::from(&mut [1.0, 2.0]), Array::from(&mut [3.0, 5.0])]);
        let r = Matrix::new(&mut [Array::from(&mut [2.0, 4.0]), Array::from(&mut [6.0, 10.0])]);

        assert_eq!(r, a.scalar(2.0));
    }

    #[test]
    fn elem_mult() {
        let a = Matrix::new(&mut [Array::from(&mut [3.0, 5.0]), Array::from(&mut [8.0, 13.0])]);
        let b = Matrix::new(&mut [Array::from(&mut [2.0, 3.0]), Array::from(&mut [5.0, 8.0])]);
        let r = Matrix::new(&mut [Array::from(&mut [6.0, 15.0]), Array::from(&mut [40.0, 104.0])]);
        
        assert_eq!(r, a.elem_mult(&b));
    }

    #[test]
    fn mult() {
        let a = Matrix::new(&mut [Array::from(&mut [1.0, 2.0]), Array::from(&mut [3.0, 4.0])]);
        let r = Matrix::new(&mut [Array::from(&mut [7.0, 10.0]), Array::from(&mut [15.0, 22.0])]);
        
        assert_eq!(r, a.mult(&a));
    }

    #[test]
    fn transpose() {
        let a = Matrix::new(&mut [Array::from(&mut [1.0, 2.0]), Array::from(&mut [3.0, 4.0])]);
        let r = Matrix::new(&mut [Array::from(&mut [1.0, 3.0]), Array::from(&mut [2.0, 4.0])]);
        
        assert_eq!(r, a.transpose());
    }

    #[test]
    fn get() {
        let a = Matrix::new(&mut [Array::from(&mut [1.0, 2.0]), Array::from(&mut [3.0, 4.0])]);
        
        assert_eq!(3.0, a.get(1, 0));
    }

    #[test]
    fn set() {
        let a = Matrix::new(&mut [Array::from(&mut [1.0, 2.0]), Array::from(&mut [3.0, 4.0])]);
        let r = Matrix::new(&mut [Array::from(&mut [1.0, 2.0]), Array::from(&mut [3.0, 8.0])]);

        a.set(8.0, 1, 1);

        assert_eq!(r, a);
    }

    #[test]
    fn iterator() {
        let a = Matrix::new(&mut [Array::from(&mut [1.0, 2.0]), Array::from(&mut [3.0, 4.0])]);
        let first = Array::from(&mut [1.0, 2.0]);
        let second = Array::from(&mut [3.0, 4.0]);

        let mut it = a.iter();

        assert_eq!(it.next(), Some(first).as_ref());
        assert_eq!(it.next(), Some(second).as_ref());
    }
}
