use crate::dataseries::DataSeries;

pub struct Column {
    pub label: String,
    pub data: DataSeries,
}

impl Column {
    pub fn new(label: &str, size: usize) -> Column {
        Column {
            label: String::from(label),
            data: DataSeries::new(size),
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn data(&self) -> &DataSeries {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use crate::column::Column;

    #[test]
    fn test_new() {
        let column = Column::new("test", 10);

        assert_eq!(column.label(), "test");
        assert_eq!(column.data.len(), 10);
    }
}