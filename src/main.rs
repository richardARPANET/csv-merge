//! `csv-merge` is a CLI tool for merging CSV files.
//!
extern crate csv;

use std::error::Error;
use std::io;
use std::env;
use std::path::Path;


fn main() -> Result<(), Box<Error>> {
    let args:Vec<String> = env::args().collect();
    let csv_paths = &args[1..];

    let mut headers_written = false;
    let mut writer = csv::Writer::from_writer(io::stdout());

    for csv_path in csv_paths
    {
        if Path::new(csv_path).exists() == false {
            let msg = format!("{} does not exist", csv_path);
            return Err(Box::from(msg));
        }

        let mut reader = csv::Reader::from_path(&csv_path)?;
        if headers_written == false {
            writer.write_record(reader.headers()?)?;
            headers_written = true;
        }
        for record in reader.records() {
            writer.write_record(&record?)?;
        }
    }
    writer.flush()?;
    return Ok(());
}


#[cfg(test)]
mod tests {
    use assert_cli;

    #[test]
    fn test_outputs_csv() {
        let expected_output = "\
            header,header2\n\
            a,123\n\
            hello,456\n\
            bob,\n\
            2-a,2-123\n\
            2-hello,2-456\n\
            2-bob,";

        let cmd = &[
            "cargo", "run", "src/test_data/1.csv", "src/test_data/2.csv"
        ];
        assert_cli::Assert::command(cmd)
            .stdout().is(expected_output)
            .unwrap();
    }
}
