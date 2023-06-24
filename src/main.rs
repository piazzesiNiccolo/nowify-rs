use csv::Reader;
use std::env;
use std::error::Error;
use std::fs::File;
fn main() {
    let args: Vec<String> = env::args().collect();

    let mut conf = read_csv();
    for result in conf.expect("FAIL").records() {
        let record = result;
        println!("{:?}", record);
    }
    match args[1].as_str() {
        "cli" => println!("cli"),
        "stats" => println!("stats"),
        "help" => println!("help"),
        _ => assert!(false),
    }
}

fn read_csv() -> Result<Reader<File>, Box<dyn Error>> {
    let file_path = "routines.csv";
    let mut rdr = csv::Reader::from_path(file_path)?;

    Ok(rdr)
}
