extern crate reqwest;
extern crate csv;

use std::fs::File;
use std::io::Write;
use std::path::Path;

const URL1: &str = "https://github.com/nogibjj/mini_project6_yabei/blob/main/data/data/performer-scores.csv?raw=true";
const URL2: &str = "https://github.com/nogibjj/mini_project6_yabei/blob/main/data/data/show-data.csv?raw=true";
const FILE_PATH1: &str = "data/performer-scores.csv";
const FILE_PATH2: &str = "data/show-data.csv";

pub fn extract() -> Result<(), Box<dyn std::error::Error>> {
    download_and_save(URL1, FILE_PATH1)?;
    download_and_save(URL2, FILE_PATH2)?;

    let mut rdr = csv::Reader::from_path(FILE_PATH2)?;
    let mut wtr = csv::Writer::from_path(FILE_PATH2)?;

    for (i, result) in rdr.records().enumerate() {
        let record = result?;
        if i < 121 {
            wtr.write_record(&record)?;
        }
    }

    Ok(())
}

fn download_and_save(url: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(url)?;
    let mut file = File::create(&Path::new(file_path))?;
    file.write_all(&response.bytes()?)?;
    Ok(())
}
