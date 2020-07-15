use anyhow::Result;
use qqt_dataset::dataset::DataSet;
use csv::{ReaderBuilder, Terminator};
use std::io::{BufReader, BufRead};

#[derive(Clone)]
pub struct Options {
    skip_lines: usize,
    quote: u8,
    delimiter: u8,
    terminator: Terminator,
    headers: bool,
}

impl Options {
    pub fn new() -> Self {
        Options {
            skip_lines: 0,
            quote: '"' as u8,
            delimiter: ',' as u8,
            terminator: Terminator::CRLF,
            headers: true,
        }
    }

    /** Number of lines to skip before parsing CSV. Default is 0 */
    pub fn skip_lines(mut self, skip_lines: usize) -> Self {
        self.skip_lines = skip_lines;

        self
    }

    /** Quote character to use. Default is double quote (") */
    pub fn quote(mut self, quote: u8) -> Self {
        self.quote = quote;

        self
    }

    /** Column delimiter to use. Default is comma (,) */
    pub fn delimiter(mut self, delimiter: u8) -> Self {
        self.delimiter = delimiter;

        self
    }

    pub fn terminator(mut self, terminator: u8) -> Self {
        self.terminator = Terminator::Any(terminator);

        self
    }

    pub fn headers(mut self, headers: bool) -> Self {
        self.headers = headers;

        self
    }
}


pub fn text_to_dataset(text: &str, options: &Options) -> Result<DataSet> {
    let mut string_buf = BufReader::new(text.as_bytes());

    for _ in 0..options.skip_lines {
        string_buf.read_line(&mut String::new());
    }

    let mut reader = ReaderBuilder::new()
        .quote(options.quote)
        .terminator(options.terminator)
        .delimiter(options.delimiter)
        .has_headers(options.headers)
        .from_reader(string_buf);

    let headers = reader.headers()?;
    let col_count = headers.len();

    let mut ds = DataSet::new(headers.iter().collect(), 0);

    for record in reader.records() {
        let values = record?.iter().map(|v| String::from(v)).collect();

        ds.add_row(values);
    }

    Ok(ds)
}

pub fn load_http(url: &str) -> Result<DataSet> {
    let mut ds = DataSet::new(vec![], 0);

    Ok(ds)
}

#[cfg(test)]
mod tests {
    use crate::{load_http, Options, text_to_dataset};

    #[test]
    fn test_text_to_dataset() {
        let text = "A,B,C\n1,2,3\n4,5,6\n7,8,9";

        let options = Options::new();

        let result = text_to_dataset(text, &options);

        let ds = result.expect("Could not parse CSV");

        assert_eq!(ds.col_count(), 3);
        assert_eq!(ds.row_count(), 3);
        assert_eq!(ds.col_label(0), "A");
        assert_eq!(ds.col_label(1), "B");
        assert_eq!(ds.col_label(2), "C");
        assert_eq!(ds.at(0, 0).value(), 1.0);
        assert_eq!(ds.at(0, 1).value(), 2.0);
        assert_eq!(ds.at(0, 2).value(), 3.0);
        assert_eq!(ds.at(1, 0).value(), 4.0);
        assert_eq!(ds.at(1, 1).value(), 5.0);
        assert_eq!(ds.at(1, 2).value(), 6.0);
        assert_eq!(ds.at(2, 0).value(), 7.0);
        assert_eq!(ds.at(2, 1).value(), 8.0);
        assert_eq!(ds.at(2, 2).value(), 9.0);
    }

    #[test]
    fn test_options_quote() {
        let text = "'A','B','C'\n'1','2','3'\n'4','5','6'\n'7','8','9'";

        let options = Options::new().quote('\'' as u8);

        let result = text_to_dataset(text, &options);

        let ds = result.expect("Could not parse CSV");

        assert_eq!(ds.col_count(), 3);
        assert_eq!(ds.row_count(), 3);
        assert_eq!(ds.col_label(0), "A");
        assert_eq!(ds.col_label(1), "B");
        assert_eq!(ds.col_label(2), "C");
        assert_eq!(ds.at(0, 0).value(), 1.0);
        assert_eq!(ds.at(0, 1).value(), 2.0);
        assert_eq!(ds.at(0, 2).value(), 3.0);
        assert_eq!(ds.at(1, 0).value(), 4.0);
        assert_eq!(ds.at(1, 1).value(), 5.0);
        assert_eq!(ds.at(1, 2).value(), 6.0);
        assert_eq!(ds.at(2, 0).value(), 7.0);
        assert_eq!(ds.at(2, 1).value(), 8.0);
        assert_eq!(ds.at(2, 2).value(), 9.0);
    }

    #[test]
    fn test_options_delimiter() {
        let text = "A;B;C\n1;2;3\n4;5;6\n7;8;9";

        let options = Options::new().delimiter(';' as u8);

        let result = text_to_dataset(text, &options);

        let ds = result.expect("Could not parse CSV");

        assert_eq!(ds.col_count(), 3);
        assert_eq!(ds.row_count(), 3);
        assert_eq!(ds.col_label(0), "A");
        assert_eq!(ds.col_label(1), "B");
        assert_eq!(ds.col_label(2), "C");
        assert_eq!(ds.at(0, 0).value(), 1.0);
        assert_eq!(ds.at(0, 1).value(), 2.0);
        assert_eq!(ds.at(0, 2).value(), 3.0);
        assert_eq!(ds.at(1, 0).value(), 4.0);
        assert_eq!(ds.at(1, 1).value(), 5.0);
        assert_eq!(ds.at(1, 2).value(), 6.0);
        assert_eq!(ds.at(2, 0).value(), 7.0);
        assert_eq!(ds.at(2, 1).value(), 8.0);
        assert_eq!(ds.at(2, 2).value(), 9.0);
    }

    #[test]
    fn test_options_terminator() {
        let text = "A,B,C;1,2,3;4,5,6;7,8,9";

        let options = Options::new().terminator(';' as u8);

        let result = text_to_dataset(text, &options);

        let ds = result.expect("Could not parse CSV");

        assert_eq!(ds.col_count(), 3);
        assert_eq!(ds.row_count(), 3);
        assert_eq!(ds.col_label(0), "A");
        assert_eq!(ds.col_label(1), "B");
        assert_eq!(ds.col_label(2), "C");
        assert_eq!(ds.at(0, 0).value(), 1.0);
        assert_eq!(ds.at(0, 1).value(), 2.0);
        assert_eq!(ds.at(0, 2).value(), 3.0);
        assert_eq!(ds.at(1, 0).value(), 4.0);
        assert_eq!(ds.at(1, 1).value(), 5.0);
        assert_eq!(ds.at(1, 2).value(), 6.0);
        assert_eq!(ds.at(2, 0).value(), 7.0);
        assert_eq!(ds.at(2, 1).value(), 8.0);
        assert_eq!(ds.at(2, 2).value(), 9.0);
    }

    #[test]
    fn test_options_skip_lines() {
        let text = "One\nTwo\nA,B,C\n1,2,3\n4,5,6\n7,8,9";

        let options = Options::new().skip_lines(2);

        let result = text_to_dataset(text, &options);

        let ds = result.expect("Could not parse CSV");

        assert_eq!(ds.col_count(), 3);
        assert_eq!(ds.row_count(), 3);
        assert_eq!(ds.col_label(0), "A");
        assert_eq!(ds.col_label(1), "B");
        assert_eq!(ds.col_label(2), "C");
        assert_eq!(ds.at(0, 0).value(), 1.0);
        assert_eq!(ds.at(0, 1).value(), 2.0);
        assert_eq!(ds.at(0, 2).value(), 3.0);
        assert_eq!(ds.at(1, 0).value(), 4.0);
        assert_eq!(ds.at(1, 1).value(), 5.0);
        assert_eq!(ds.at(1, 2).value(), 6.0);
        assert_eq!(ds.at(2, 0).value(), 7.0);
        assert_eq!(ds.at(2, 1).value(), 8.0);
        assert_eq!(ds.at(2, 2).value(), 9.0);
    }



    // #[test]
    fn test_load_http() {
        let url = "https://data.giss.nasa.gov/gistemp/graphs/graph_data/Global_Mean_Estimates_based_on_Land_and_Ocean_Data/graph.csv";

        let result = load_http(url);

        let ds = result.expect("Could not load dataset");

        assert_eq!(ds.col_count(), 3);
    }
}
