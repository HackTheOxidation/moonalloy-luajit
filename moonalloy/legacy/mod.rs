pub mod reader;

use std::fmt::*;
use std::alloc::{alloc, Layout};
use std::ops::Deref;
use std::os::raw::c_char;
use std::ffi::{CStr, CString};

#[derive(Debug, Clone)]
#[repr(C)]
pub enum DataCell {
    Int(i32),
    Float(f64),
    Bool(bool),
    Str(*mut c_char),
    Empty,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct DataRow {
    length: usize,
    entries: *const DataCell,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct DataTable {
    rows: usize,
    cols: usize,
    labels: *mut *mut c_char,
    data: *const DataRow,
}

impl DataCell {
    pub fn from(string: String) -> DataCell {
        match string.parse::<i32>() {
            Ok(num) => return DataCell::Int(num),
            Err(_) => (),
        };

        match string.parse::<f64>() {
            Ok(num) => return DataCell::Float(num),
            Err(_) => (),
        };

        match string.parse::<bool>() {
            Ok(b) => return DataCell::Bool(b),
            Err(_) => (),
        };

        if string.as_str() == "" {
            return DataCell::Empty;
        } else {
            let mut string = string + "\0";
            let c_string = string.as_mut_ptr() as *mut i8;
            return DataCell::Str(c_string);
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Int(num) => num.to_string(),
            Self::Float(num) => num.to_string(),
            Self::Bool(b) => b.to_string(),
            Self::Str(cs) => {
                let c_string = unsafe {
                    CStr::from_ptr(*cs)
                };
                c_string.to_str().unwrap().to_string()
            },
            Self::Empty => String::from("-Empty-")
        }
    }
}

impl DataRow {
    fn new(entries: &[DataCell]) -> DataRow {
        DataRow {
            length: entries.len(),
            entries: entries.as_ptr(),
        }
    }

    fn len(&self) -> usize {
        self.length
    }

    fn get(&self, index: usize) -> DataCell {
        assert!(index < self.length);

        let entries = unsafe {
            std::slice::from_raw_parts(self.entries, self.length)
        };

        entries[index].clone()
    }

    pub fn to_string(&self) -> String {
        let cells = unsafe {
            std::slice::from_raw_parts(self.entries, self.len())
        };

        format!("{:?}", cells)
    }
}

impl DataTable {
    pub fn new(data: &[&[DataCell]], labels: Vec<String>) -> DataTable {
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

        let mut label_ptrs: Vec<*mut c_char> = Vec::with_capacity(labels.len());

        labels.iter().for_each(|elem| {
            let mut c_string = elem.to_owned() + "\0";
            let ptr = c_string.as_mut_ptr() as *mut i8;
            std::mem::forget(c_string);
            label_ptrs.push(ptr);
        });

        let ptrs = label_ptrs.as_mut_ptr();
        std::mem::forget(label_ptrs);

        DataTable {
            rows: data.len(),
            cols: data[0].len(),
            labels: ptrs,
            data: data_rows.as_ptr(),
        }
    }

    pub fn from(slice: &[DataRow], labels: Vec<String>) -> DataTable {

        let mut label_ptrs: Vec<*mut c_char> = Vec::with_capacity(labels.len());

        labels.iter().for_each(|elem| {
            let mut c_string = elem.to_owned() + "\0";
            let ptr = c_string.as_mut_ptr() as *mut i8;
            std::mem::forget(c_string);
            label_ptrs.push(ptr);
        });

        let ptrs = label_ptrs.as_mut_ptr();
        std::mem::forget(label_ptrs);

        DataTable {
            rows: slice.len(),
            cols: slice[0].len(),
            labels: ptrs,
            data: slice.as_ptr(),
        }
    }

    fn is_valid_slice(slice: &[&[DataCell]]) -> bool {
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
            std::slice::from_raw_parts(self.data, self.rows)
        };

        data_rows[i].get(j)
    }

    pub fn get_labels(&self) -> Vec<String> {
        let v = unsafe {
            Vec::from_raw_parts(self.labels, self.cols, self.cols)
        };

        let mut strings: Vec<String> = Vec::with_capacity(self.cols);

        v.iter().for_each(|ptr| {
            let string = unsafe {
                CString::from_raw(ptr.clone())
            };
            strings.push(String::from(string.to_owned().to_str().unwrap()));
        });
        std::mem::forget(v);

        strings
    }

    pub fn get_labels_as_string(&self) -> String {
        let labels = self.get_labels();

        let res = format!("{:?}", labels).to_string();
        std::mem::forget(labels);
        res
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
        std::mem::forget(labels);

        res
    }

    pub fn to_raw(dt: DataTable) -> *const DataTable {
        Box::into_raw(Box::new(dt))
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


impl Deref for DataTable {
    type Target = [DataRow];
    fn deref(&self) -> &[DataRow] {
        unsafe {
            std::slice::from_raw_parts(self.data, self.rows)
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
    use super::*;

    #[test]
    fn create_new_datatable() {
        let dt = DataTable::new(
            &[
                &[DataCell::Float(1.0), DataCell::Float(2.0)],
                &[DataCell::Float(3.0), DataCell::Float(4.0)]
            ],
            vec![String::from("attr1"), String::from("attr2")]
            );

        let result = match dt.get(1,0) {
            DataCell::Float(num) => num,
            _ => panic!("Not a float"),
        };

        assert_eq!(3.0, result);
    }
}
