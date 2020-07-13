use crate::dataseries::{DataSeriesType, DataSeries};
use std::ops::Index;

/** A set (column) of values */
pub struct Values {
    pub values: Vec<f64>
}

impl Values {
    fn new(size: usize) -> Values {
        Values {
            values: vec![0.0; size]
        }
    }

    fn iter(&self) -> ValuesIter {
        ValuesIter {
            index: 0,
            values: self,
        }
    }
}

impl Index<usize> for Values {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl DataSeries for Values {
    fn series_type(&self) -> DataSeriesType {
        DataSeriesType::Values
    }

    fn len(&self) -> usize {
        self.values.len()
    }
}

pub struct ValuesIter<'a> {
    values: &'a Values,
    index: usize,
}

impl<'a> Iterator for ValuesIter<'a> {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.values.len() {
            let value = self.values[self.index];
            self.index += 1;
            Some(value)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::values::Values;
    use crate::dataseries::DataSeries;

    #[test]
    fn test_new() {
        let values = Values::new(10);

        assert_eq!(values.len(), 10);

        assert_eq!(values[0], 0.0);
    }

    #[test]
    fn test_iter() {
        let values = Values::new(10);

        for value in values.iter() {
            assert_eq!(value, 0.0);
        }
    }
}
