pub mod reader;

use std::fmt::*;
use std::alloc::{alloc, Layout};
use std::ops::Deref;

#[derive(Debug, Clone)]
#[repr(C)]
pub enum DataCell {
    Int(i32),
    Float(f64),
    Bool(bool),
    Str(String),
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
    labels: *const String,
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
            return DataCell::Str(string);
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Int(num) => num.to_string(),
            Self::Float(num) => num.to_string(),
            Self::Bool(b) => b.to_string(),
            Self::Str(cs) => String::from(cs.as_str()),
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
    pub fn new(data: &[&[DataCell]], labels: &[String]) -> DataTable {
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
            labels: labels.as_ptr(),
            data: data_rows.as_ptr(),
        }
    }

    pub fn from(slice: &[DataRow], labels: &[String]) -> DataTable {
        DataTable {
            rows: slice.len(),
            cols: slice[0].len(),
            labels: labels.as_ptr(),
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

    pub fn get_labels(&self) -> &[String] {
        unsafe {
            std::slice::from_raw_parts(self.labels, self.cols)
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

    pub fn to_raw(dt: DataTable) -> *mut DataTable {
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
            &mut [
                &mut [DataCell::Float(1.0), DataCell::Float(2.0)],
                &mut [DataCell::Float(3.0), DataCell::Float(4.0)]
            ],
            &mut [String::from("attr1"), String::from("attr2")]
            );

        let result = match dt.get(1,0) {
            DataCell::Float(num) => num,
            _ => panic!("Not a float"),
        };

        assert_eq!(3.0, result);
    }
}
