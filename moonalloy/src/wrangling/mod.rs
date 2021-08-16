pub mod reader;

use std::ffi::CString;
use std::fmt::*;
use std::alloc::{alloc, Layout};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone)]
#[repr(C)]
pub enum DataCell {
    Int(i32),
    Float(f64),
    Bool(bool),
    Str(CString),
    Empty,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct DataRow {
    length: usize,
    entries: *mut DataCell,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct DataTable {
    rows: usize,
    cols: usize,
    labels: *mut CString,
    data: *mut DataRow,
}

impl DataCell {
    pub fn to_string(&self) -> String {
        match self {
            Self::Int(num) => num.to_string(),
            Self::Float(num) => num.to_string(),
            Self::Bool(b) => b.to_string(),
            Self::Str(cs) => String::from(cs.to_str().unwrap()),
            Self::Empty => String::from("-Empty-")
        }
    }
}

impl DataRow {
    fn new(entries: &mut [DataCell]) -> DataRow {
        DataRow {
            length: entries.len(),
            entries: entries.as_mut_ptr(),
        }
    }

    fn len(&self) -> usize {
        self.length
    }

    fn get(&self, index: usize) -> DataCell {
        assert!(index < self.length);

        let entries = unsafe {
            std::slice::from_raw_parts_mut(self.entries, self.length)
        };

        entries[index].clone()
    }

    fn set(&self, val: DataCell, index: usize) {
        assert!(index < self.length);

        let entries = unsafe {
            std::slice::from_raw_parts_mut(self.entries, self.length)
        };
        
        entries[index] = val;
    }

    pub fn to_string(&self) -> String {
        let cells = unsafe {
            std::slice::from_raw_parts(self.entries, self.len())
        };

        format!("{:?}", cells)
    }
}

impl DataTable {
    pub fn new(data: &mut [&mut [DataCell]], labels: &mut [CString]) -> DataTable {
        assert!(DataTable::is_valid_slice(data));
        assert!(labels.len() == data[0].len());
         
        let data_rows = unsafe {
            let layout = Layout::array::<DataRow>(data.len()).unwrap();
            let ptr = alloc(layout);
            std::slice::from_raw_parts_mut(ptr as *mut DataRow, data.len())
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

    fn is_valid_slice(slice: &mut [&mut [DataCell]]) -> bool {
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

    pub fn get(&self, i: usize, j: usize) -> DataCell {
        assert!(i < self.rows);
        assert!(j < self.cols);

        let data_rows = unsafe {
            std::slice::from_raw_parts_mut(self.data, self.rows)
        };

        data_rows[i].get(j)
    }

    pub fn set(&self, val: DataCell, i: usize, j: usize) {
        assert!(i < self.rows);
        assert!(j < self.cols);

        let data_rows = unsafe {
            std::slice::from_raw_parts_mut(self.data, self.rows)
        };

        data_rows[i].set(val, j);
    }

    pub fn get_labels(&self) -> &mut [CString] {
        unsafe {
            std::slice::from_raw_parts_mut(self.labels, self.cols)
        }
    }

    pub fn to_string(&self) -> String {
        let labels = self.get_labels();
        let rows = unsafe {
            std::slice::from_raw_parts(self.data, self.rows)
        };

        let mut res = String::from("");
        res = res + format!("{:?}\n", labels).as_str();
        
        for elem in rows {
            res = res + format!("{}\n", elem).as_str();
        }

        res
    }
}

impl Deref for DataRow {
    type Target = [DataCell];
    fn deref(&self) -> &[DataCell] {
        unsafe {
            std::slice::from_raw_parts(self.entries, self.len())
        }
    }
}

impl DerefMut for DataRow {
    fn deref_mut(&mut self) -> &mut [DataCell] {
        unsafe {
            std::slice::from_raw_parts_mut(self.entries, self.len())
        }
    }
}

impl Deref for DataTable {
    type Target = [DataRow];
    fn deref(&self) -> &[DataRow] {
        unsafe {
            std::slice::from_raw_parts(self.data, self.rows)
        }
    }
}

impl DerefMut for DataTable {
    fn deref_mut(&mut self) -> &mut [DataRow] {
        unsafe {
            std::slice::from_raw_parts_mut(self.data, self.rows)
        }
    }
}

impl Display for DataCell {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.to_string())
    }
}

impl Display for DataRow {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.to_string())
    }
}

impl Display for DataTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod test {

}
