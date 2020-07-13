use std::fmt::{Debug};

pub type Number = f64;

#[derive(Clone, Debug, PartialEq)]
pub struct  DataValue {
    raw: String,
    value: Number,
    is_value: bool,
}

impl DataValue {
    pub fn new(raw: &str) -> DataValue {
        let mut value = 0.0;
        let mut is_value = false;

        if let Ok(v) = raw.parse::<Number>() {
            value = v;
            is_value = true;
        };

        DataValue {
            raw: String::from(raw),
            value,
            is_value,
        }
    }

    pub fn new_value(value: Number) -> DataValue {
        DataValue {
            raw: value.to_string(),
            value,
            is_value: true,
        }
    }

    pub fn null() -> DataValue {
        DataValue::new("")
    }

    pub fn raw(&self) -> &str {
        &self.raw
    }

    pub fn trim(&self) -> DataValue {
        DataValue::new(self.raw().trim())
    }

    pub fn value(&self) -> Number {
        self.value
    }

    pub fn is_value(&self) -> bool {
        self.is_value
    }
}

#[cfg(test)]
mod tests {
    use crate::datavalue::DataValue;

    #[test]
    fn test_data_value_new() {
        let value = DataValue::new("test");

        assert_eq!(value.raw(), "test");
        assert_eq!(value.value(), 0.0);
        assert_eq!(value.is_value(), false);

        let value = DataValue::new("1.0");

        assert_eq!(value.raw(), "1.0");
        assert_eq!(value.value(), 1.0);
        assert_eq!(value.is_value(), true);
    }

    fn test_null() {
        let null = DataValue::null();

        assert_eq!(null.raw(), "");
        assert_eq!(null.value(), 0.0);
        assert_eq!(null.is_value, false);
    }

    fn test_trim() {
        let value = DataValue::new("").trim();

        assert_eq!(value.raw(), "");

        let value = DataValue::new("  ").trim();

        assert_eq!(value.raw(), "");

        let value = DataValue::new(" a ").trim();

        assert_eq!(value.raw(), "a");

        let value = DataValue::new(" 1.0 ").trim();

        assert_eq!(value.raw(), "1.0");
        assert_eq!(value.value(), 1.0);
        assert_eq!(value.is_value(), true);
    }
}
