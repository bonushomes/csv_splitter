extern crate csv;
extern crate clap;

use std::fs::File;
use std::io::{BufWriter, Write};
use clap::{Arg, App};

fn split_csv(input_file: &str, lines_per_file: usize, prefix: &str) -> std::io::Result<()> {
    let input = File::open(input_file)?;
    let mut rdr = csv::Reader::from_reader(input);
    let headers = rdr.headers()?.clone();
    let mut output_file_num = 1;
    let mut output_file = File::create(format!("{}_{}.csv", prefix, output_file_num))?;
    let mut writer = csv::Writer::from_writer(output_file);
    writer.write_record(&headers)?;

    let mut lines = 0;
    let mut invalid_lines = Vec::new();
    for result in rdr.records() {
        match result {
            Ok(record) => {
                lines += 1;
                if lines % lines_per_file == 0 {
                    output_file_num += 1;
                    output_file = File::create(format!("{}_{}.csv", prefix, output_file_num))?;
                    writer = csv::Writer::from_writer(output_file);
                    writer.write_record(&headers)?;
                }
                writer.write_record(&record)?;
            },
            Err(e) => {
                invalid_lines.push(e.to_string());
            }
        }
    }

    if invalid_lines.len() > 0 {
        let mut failed_file = BufWriter::new(File::create("failed.txt")?);
        for line in invalid_lines {
            writeln!(failed_file, "{}", line)?;
        }
    }
    Ok(())
}

fn main() {
    let matches = App::new("csv-split")
        .version("1.0")
        .author("Author Name")
        .about("Splits a large CSV file into smaller files")
        .arg(Arg::with_name("input")
            .help("Sets the input file to use")
            .required(true)
            .index(1))
        .arg(Arg::with_name("lines")
            .help("Sets the number of lines per output file")
            .required(true)
            .index(2))
        .arg(Arg::with_name("prefix")
            .help("Sets the prefix of the output file names")
            .required(true)
            .index(3))
        .get_matches();

    let input_file = matches.value_of("input").unwrap();
    let lines_per_file = matches.value_of("lines").unwrap().parse::<usize>().unwrap();
    let prefix = matches.value_of("prefix").unwrap();

    split_csv(input_file, lines_per_file, prefix).unwrap();
}
