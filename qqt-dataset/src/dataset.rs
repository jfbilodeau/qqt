use crate::column::Column;
use crate::datavalue::DataValue;

pub struct DataSet {
    pub columns: Vec<Column>
}

impl DataSet {
    pub fn new(labels: Vec<&str>, row_count: usize) -> DataSet{
        let mut ds = DataSet {
            columns: vec![],
        };

        for label in labels {
            ds.columns.push(Column::new(label, row_count));
        }

        ds
    }

    pub fn col_count(&self) -> usize {
        self.columns.len()
    }

    pub fn row_count(&self) -> usize {
        if self.columns.len() != 0 {
            self.columns[0].data.len()
        } else {
            0
        }
    }

    pub fn col_label(&self, index: usize) -> &str {
        self.columns[index].label()
    }

    pub fn add_row(&mut self, values: Vec<String>) {
        for i in 0..self.col_count() {
            self.columns[i].data.add_row(&values[i]);
        }
    }

    pub fn at(&self, row: usize, col: usize) -> &DataValue {
        &self.columns[col].data[row]
    }
}

#[cfg(test)]
mod tests {
    use crate::dataset::DataSet;

    #[test]
    fn test_new() {
        let ds = DataSet::new(vec!["A", "B", "C"], 10);

        assert_eq!(ds.col_count(), 3);
        assert_eq!(ds.row_count(), 10);
        assert_eq!(ds.col_label(0), "A");
        assert_eq!(ds.col_label(1), "B");
        assert_eq!(ds.col_label(2), "C");
    }

    #[test]
    fn test_add_row() {
        let mut ds = DataSet::new(vec!["A", "B", "C"], 5);

        ds.add_row(vec![String::from("1"), String::from("2"), String::from("3")]);

        assert_eq!(ds.row_count(), 6);
        assert_eq!(ds.at(5, 0).value(), 1.0);
        assert_eq!(ds.at(5, 1).value(), 2.0);
        assert_eq!(ds.at(5, 2).value(), 3.0);
    }
}