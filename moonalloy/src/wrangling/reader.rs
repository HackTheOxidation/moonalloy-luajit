use crate::wrangling::*;
use std::fs;
use std::alloc::{alloc, Layout};

pub fn read_csv(filename: String) -> DataTable {
    let csv = CSV::read_from_file(filename);

    let (rows, cols) = csv.dimensions();

    let data = unsafe {
        let layout = Layout::array::<DataRow>(rows).unwrap();
        let ptr = alloc(layout);
        std::slice::from_raw_parts_mut(ptr as *mut DataRow, rows)
    };

    for i in 0..rows {
        let dr = unsafe {
            let layout = Layout::array::<DataCell>(cols).unwrap();
            let ptr = alloc(layout);
            std::slice::from_raw_parts_mut(ptr as *mut DataCell, cols)
        };

        for j in 0..cols {
            dr[j] = DataCell::from(csv.get(i, j));
        }

        data[i] = DataRow::new(dr);
    }

    DataTable::from(data, csv.labels)
}

pub struct CSV {
    pub content: Vec<Vec<String>>,
    pub labels: Vec<String>,
}

impl CSV {
    pub fn read_from_file(filename: String) -> CSV {
        let from_file = fs::read_to_string(filename).unwrap();

        if from_file.is_empty() {
            panic!("File is empty.");
        }

        let mut lines: Vec<Vec<String>> = from_file
            .split("\n")
            .into_iter()
            .map(|line| line.to_string().to_string())
            .map(|line| line.split(",").into_iter().map(|elem| elem.to_string()).collect())
                .collect();

        lines.pop();

        if !CSV::verify_content(lines.clone()) {
            panic!("File has non-rectangular data.");
        }

        let labels = lines.remove(0);
        let content = lines;

        CSV {
            content,
            labels,
        }
    }

    fn verify_content(lines: Vec<Vec<String>>) -> bool {
        let label_len = lines[0].len();

        for line in lines {
            println!("line = {:?}, len() = {}", line, line.len());
            if line.len() != label_len {
                return false;
            }
        }

        true
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();

        for i in 0..self.labels.len() {
            if i == self.labels.len() - 1 {
                result += &self.labels[i];
            } else {
                result += &self.labels[i];
                result += ", ";
            }
        }

        result += "\n";

        for line in &self.content {
            for i in 0..line.len() {
                if i == line.len() - 1 {
                    result += &line[i];
                } else {
                    result += &line[i];
                    result += ", ";
                }
            }
            result += "\n";
        }

        result
    }

    pub fn get(&self, i: usize, j: usize) -> String {
        self.content[i][j].clone()
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.content.len(), self.labels.len())
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_from_csv() {
        let dt = read_csv("test.csv".to_string());
        let result = match dt.get(1, 0) {
            DataCell::Float(num) => num,
            _ => panic!("Not a float!"),
        };

        assert_eq!(3.0, result);
    }
}
