use csv::Error;
use serde::{Deserialize, Serialize};
use std::{env, fs::OpenOptions, process};

#[derive(Debug, Serialize, Deserialize)]
struct SampleCsv {
    string: String,
    integer: usize,
    float: f64,
    boolean: bool,
}

fn read_csv() -> Result<(), Box<Error>> {
    let file_path = env::args_os()
        .nth(1)
        .expect("Give the relative path of the csv file as an argument.");
    let mut rdr = csv::Reader::from_path(file_path)?;
    for result in rdr.deserialize() {
        let record: SampleCsv = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn write_csv() -> Result<(), Box<Error>> {
    let file_path = env::args_os()
        .nth(1)
        .expect("Give the relative path of the csv file as an argument.");
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)
        .expect("Could not open file");
    let mut wtr = csv::Writer::from_writer(file);

    //header
    wtr.write_record(["Name", "Age", "Sex", "Height"])?;
    //data
    wtr.write_record(["山田花子", "23", "female", "1.56"])?;
    wtr.write_record(["山田太郎", "25", "male", "1.76"])?;
    wtr.write_record(["田中三郎", "70", "male", "1.66"])?;
    wtr.flush().unwrap();

    Ok(())
}

fn main() {
    if let Err(err) = read_csv() {
        println!("{}", err);
        process::exit(1);
    }

    // if let Err(err) = write_csv() {
    //     println!("{}", err);
    //     process::exit(1);
    // }
}
