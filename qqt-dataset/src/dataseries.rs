use crate::datavalue::{DataValue, Number};
use std::ops::Index;
use std::f64::NAN;

pub struct DataSeries {
    data: Vec<DataValue>,
}

impl DataSeries {
    pub fn new(size: usize) -> DataSeries {
        DataSeries {
            data: vec![DataValue::null(); size]
        }
    }

    pub fn seq(size: usize, start_at: Number, increment: Number) -> DataSeries {
        let mut data = DataSeries::new(size);

        let mut current = start_at;

        for i in 0..size {
            data.data[i] = DataValue::new_value(current);

            current += increment;
        }

        data
    }

    pub fn from(values: Vec<&str>) -> DataSeries {
        let mut seq = DataSeries::new(values.len());

        for i in 0..values.len() {
            seq.data[i] = DataValue::new(&values[i]);
        }

        seq
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn count_not_null(&self) -> usize {
        self.data.iter().filter(|v| v.raw() != "").count()
    }

    pub fn count_val(&self) -> usize {
        self.data.iter().filter(|v| v.is_value()).count()
    }

    pub fn sum(&self) -> Number {
        let mut sum = 0.0;

        self.data.iter().for_each(|v| sum += v.value());

        sum
    }

    pub fn mean(&self) -> Number {
        if self.len() > 0 {
            self.sum() / self.count_val() as f64
        } else {
            NAN
        }
    }

    pub fn var_p(&self) -> Number {
        if self.len() > 0 {
            let mean = self.mean();
            let mut var = 0.0;

            for v in self.data.iter() {
                if v.is_value() {
                    var += (v.value() - mean).powi(2);
                }
            }

            var / self.count_val() as Number
        } else {
            0.0
        }
    }

    pub fn stddev_p(&self) -> Number {
        self.var_p().sqrt()
    }


    pub fn var_s(&self) -> Number {
        if self.len() > 0 {
            let mean = self.mean();
            let mut var = 0.0;

            for v in self.data.iter() {
                if v.is_value() {
                    var += (v.value() - mean).powi(2);
                }
            }

            var / (self.count_val() - 1) as Number
        } else {
            0.0
        }
    }

    pub fn stddev_s(&self) -> Number {
        self.var_s().sqrt()
    }

    pub fn trim(&self) -> DataSeries {
        let mut data = DataSeries::new(self.len());

        for i in 0..self.len() {
            data.data[i] = self.data[i].trim();
        }

        data
    }

    pub fn add_row(&mut self, value: &str) {
        self.data.push(DataValue::new(value));
    }
}

impl Index<usize> for DataSeries {
    type Output = DataValue;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

#[cfg(test)]
mod tests {
    use crate::dataseries::DataSeries;
    use crate::datavalue::DataValue;

    #[test]
    fn test_new() {
        let series = DataSeries::new(10);

        assert_eq!(series.len(), 10);
        assert_eq!(series[0], DataValue::null());
    }

    #[test]
    fn test_seq() {
        let s1 = DataSeries::seq(5, 1.0, 1.0);

        assert_eq!(s1.len(), 5);
        assert_eq!(s1[0], DataValue::new_value(1.0));
        assert_eq!(s1[1], DataValue::new_value(2.0));
        assert_eq!(s1[2], DataValue::new_value(3.0));
        assert_eq!(s1[3], DataValue::new_value(4.0));
        assert_eq!(s1[4], DataValue::new_value(5.0));

        let s2 = DataSeries::seq(6, 2.0, -0.5);

        assert_eq!(s2.len(), 6);
        assert_eq!(s2[0], DataValue::new_value(2.0));
        assert_eq!(s2[1], DataValue::new_value(1.5));
        assert_eq!(s2[2], DataValue::new_value(1.0));
        assert_eq!(s2[3], DataValue::new_value(0.5));
        assert_eq!(s2[4], DataValue::new_value(0.0));
        assert_eq!(s2[5], DataValue::new_value(-0.5));
    }

    #[test]
    fn test_from() {
        let s1 = DataSeries::from(vec!["a", "b", "", "1.0", "2"]);

        assert_eq!(s1[0], DataValue::new("a"));
        assert_eq!(s1[1], DataValue::new("b"));
        assert_eq!(s1[2], DataValue::new(""));
        assert_eq!(s1[3], DataValue::new("1.0"));
        assert_eq!(s1[4], DataValue::new("2"));
    }

    #[test]
    fn test_sum() {
        let s1 = DataSeries::seq(5, 1.0, 1.0);

        let sum = s1.sum();

        assert_eq!(sum, 15.0);
    }

    #[test]
    fn test_mean() {
        let s1 = DataSeries::seq(5, 1.0, 1.0);

        let mean = s1.mean();

        assert_eq!(mean, 3.0);

        let s1 = DataSeries::from(vec!["a", " 2.0 ", "1", "", "2.0", "b", "3.00"]);

        let mean = s1.mean();

        assert_eq!(mean, 2.0);
    }

    #[test]
    fn test_count_not_null() {
        let s1 = DataSeries::from(vec!["a", "b", "", " ", "1", "2"]);

        let count = s1.count_not_null();

        assert_eq!(count, 5);
    }

    #[test]
    fn test_count_value() {
        let s1 = DataSeries::from(vec!["a", "b", "", " ", "1.0", "2"]);

        let count = s1.count_val();

        assert_eq!(count, 2);
    }

    #[test]
    fn test_trim() {
        let s1 = DataSeries::from(vec![ " a ", "b", " ", " 1.0 "]);

        let s2 = s1.trim();

        assert_eq!(s2[0].raw(), "a");
        assert_eq!(s2[1].raw(), "b");
        assert_eq!(s2[2].raw(), "");
        assert_eq!(s2[3].value(), 1.0);
    }

    #[test]
    fn test_var_p() {
        let s1 = DataSeries::seq(10, 1.0, 1.0);

        let var_p = s1.var_p();

        assert_eq!(var_p, 8.25);
    }

    #[test]
    fn test_stddev_p() {
        let s1 = DataSeries::seq(10, 1.0, 1.0);

        let stddev_p = s1.stddev_p();

        assert_eq!(stddev_p, 2.8722813232690143);
    }

    #[test]
    fn test_var_s() {
        let s1 = DataSeries::seq(10, 1.0, 1.0);

        let var_s = s1.var_s();

        assert_eq!(var_s, 9.166666666666666);
    }

    #[test]
    fn test_stddev_s() {
        let s1 = DataSeries::seq(10, 1.0, 1.0);

        let stddev_s = s1.stddev_s();

        assert_eq!(stddev_s, 3.0276503540974917);
    }

    #[test]
    fn test_add_row() {
        let mut s1 = DataSeries::new(10);

        s1.add_row("11");

        assert_eq!(s1.len(), 11);
        assert_eq!(s1[10].raw(), "11");
    }
}