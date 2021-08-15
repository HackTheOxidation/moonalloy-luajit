use std::ffi::CString;
use std::fmt::*;
use std::alloc::{alloc, Layout};

#[derive(Debug, Clone)]
#[repr(C)]
pub struct DataTable<T: Clone> {
    pub rows: usize,
    pub cols: usize,
    pub labels: *mut CString,
    data: *mut DataRow<T>,
}

#[derive(Debug, Clone)]
#[repr(C)]
struct DataRow<T: Clone> {
    length: usize,
    entries: *mut T,
}

impl<T: Clone> DataRow<T> {
    fn new(entries: &mut [T]) -> DataRow<T> {
        DataRow {
            length: entries.len(),
            entries: entries.as_mut_ptr(),
        }
    }

    fn get(&self, index: usize) -> T {
        assert!(index < self.length);

        let entries = unsafe {
            std::slice::from_raw_parts_mut(self.entries, self.length)
        };

        entries[index].clone()
    }

    fn set(&self, val: T, index: usize) {
        assert!(index < self.length);

        let entries = unsafe {
            std::slice::from_raw_parts_mut(self.entries, self.length)
        };
        
        entries[index] = val;
    }
}

impl<T: Clone> DataTable<T> {
    pub fn new(data: &mut [&mut [T]], labels: &mut [CString]) -> DataTable<T> {
        assert!(DataTable::is_valid_slice(data));
        assert!(labels.len() == data[0].len());
         
        let data_rows = unsafe {
            let layout = Layout::array::<DataRow<T>>(data.len()).unwrap();
            let ptr = alloc(layout);
            std::slice::from_raw_parts_mut(ptr as *mut DataRow<T>, data.len())
        };

        for i in 0..data.len() {
            data_rows[i] = DataRow::new(data[i]);
        }

        DataTable {
            rows: data.len(),
            cols: data[0].len(),
            labels: labels.as_mut_ptr(),
            data: data_rows.as_mut_ptr(),
        }
    }

    fn is_valid_slice(slice: &mut [&mut [T]]) -> bool {
        if slice.is_empty() {
            return false;
        }

        let expected_length = slice[0].len();
        for i in 1..slice.len() {
            if expected_length != slice[i].len() {
                return false;
            }
        }

        true
    }

    pub fn get(&self, i: usize, j: usize) -> T {
        assert!(i < self.rows);
        assert!(j < self.cols);

        let data_rows = unsafe {
            std::slice::from_raw_parts_mut(self.data, self.rows)
        };

        data_rows[i].get(j)
    }

    pub fn set(&self, val: T, i: usize, j: usize) {
        assert!(i < self.rows);
        assert!(j < self.cols);

        let data_rows = unsafe {
            std::slice::from_raw_parts_mut(self.data, self.rows)
        };

        data_rows[i].set(val, j);
    }
}

#[cfg(test)]
mod test {

}
