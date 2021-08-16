extern crate csv;

use crate::wrangling::{DataTable, DataRow, DataCell};
use std::fs;
use csv::{Reader, StringRecord};
use std::alloc::{alloc, Layout};
use std::ffi::CString;

pub fn read_csv(filename: String) -> DataTable {
    let reader = fs::read_to_string(filename).unwrap();

    let mut rdr = Reader::from_reader(reader.as_str().as_bytes());

    let records: Vec<StringRecord> = rdr.records().into_iter().map(|elem| elem.unwrap()).collect();

    let labels = unsafe {
        let layout = Layout::array::<CString>(records.len()).unwrap();
        let ptr = alloc(layout);
        std::slice::from_raw_parts_mut(ptr as *mut CString, records.len())
    };

    let data = unsafe {
        let layout = Layout::array::<DataRow>(records.len()).unwrap();
        let ptr = alloc(layout);
        std::slice::from_raw_parts_mut(ptr as *mut DataRow, records.len())
    };

    for i in 0..records.len() {
        if i == 0 {
            for j in 0..records[i].len() {
                labels[j] = CString::new(records[i].get(j).unwrap()).unwrap();
            }
        } else {
            let row = unsafe {
                let layout = Layout::array::<DataCell>(records[i].len()).unwrap();
                let ptr = alloc(layout);
                std::slice::from_raw_parts_mut(ptr as *mut DataCell, records[i].len())
            };

            for j in 0..records[i].len() {
                row[j] = DataCell::from(records[i].get(j).unwrap().to_string());
            }

            data[i] = DataRow::new(row);
        }
    }

    DataTable::from(data, labels)
}
