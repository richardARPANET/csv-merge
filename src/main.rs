//! `csv-merge` is a CLI tool for merging CSV files.
//!
extern crate csv;

use std::error::Error;
use std::io;
use std::env;


fn main() -> Result<(), Box<Error>> {
    let args:Vec<String> = env::args().collect();
    let csv_paths = &args[1..];

    let mut headers_written = false;
    let mut writer = csv::Writer::from_writer(io::stdout());

    for csv_path in csv_paths
    {
        let mut reader;
        match csv::Reader::from_path(&csv_path) {
            Ok(val) => reader = val,
            Err(_) => return Err(Box::from(
                format!("{} does not exist", csv_path)
            )),
        }
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


#[macro_use]
extern crate galvanic_test;


test_suite! {
    use super::*;
    use assert_cli;

    fn create_test_csv(label: &str) -> Result<String, io::Error> {
        let csv_path = String::from(format!("/tmp/{}.csv", label));
        let mut wtr = csv::WriterBuilder::new().from_path(&csv_path)?;
        wtr.write_record(&["header", "header2"])?;
        wtr.write_record(
            &[format!("csv{}_value{}_a", label, label),
            format!("csv{}_anothervalue{}_a", label, label)]
        )?;
        wtr.write_record(
            &[format!("csv{}_value{}_b", label, label),
            format!("csv{}_anothervalue{}_b", label, label)]
        )?;
        wtr.flush()?;
        return Ok(csv_path)
    }

    test test_outputs_csv() {
        let csv_path1;
        let csv_path2;

        match create_test_csv("1") {
            Ok(val) => csv_path1 = val,
            Err(e) => panic!(e),
        }
        match create_test_csv("2") {
            Ok(val) => csv_path2 = val,
            Err(e) => panic!(e),
        }

        let expected_output = "\
            header,header2\n\
            csv1_value1_a,csv1_anothervalue1_a\n\
            csv1_value1_b,csv1_anothervalue1_b\n\
            csv2_value2_a,csv2_anothervalue2_a\n\
            csv2_value2_b,csv2_anothervalue2_b\n";

        let cmd = &["cargo", "run", &csv_path1, &csv_path2];
        assert_cli::Assert::command(cmd)
            .stdout().is(expected_output)
            .unwrap();
    }

    test test_input_csvs_do_not_exist() {
        let cmd = &["cargo", "run", "does-not-exist.csv", "does-not-exist.csv"];
        assert_cli::Assert::command(cmd)
            .fails()
            .unwrap();
    }
}
