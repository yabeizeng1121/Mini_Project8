extern crate csv;
extern crate rusqlite;

use rusqlite::{params, Connection, Result};
use std::fs::File;

const DATASET1: &str = "data/performer-scores.csv";
const DATASET2: &str = "data/show-data.csv";

pub fn transform_load() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("database.db")?;

    // Create performerscoresDB table if not exists
    conn.execute(
        "CREATE TABLE IF NOT EXISTS performerscoresDB (
            Performer TEXT,
            Score_per_year REAL,
            Total_score REAL,
            Show TEXT
        )",
        [],
    )?;

    // Insert data into performerscoresDB
    let mut rdr = csv::Reader::from_reader(File::open(DATASET1)?);
    for result in rdr.records() {
        let record = result?;
        conn.execute(
            "INSERT INTO performerscoresDB (Performer, Score_per_year, Total_score, Show) VALUES (?1, ?2, ?3, ?4)",
            params![&record[0], &record[1].parse::<f32>()?, &record[2].parse::<f32>()?, &record[3]],
        )?;
    }

    // Create showdataDB table if not exists
    conn.execute(
        "CREATE TABLE IF NOT EXISTS showdataDB (
            Performer TEXT,
            Show TEXT,
            Show_Start TEXT,
            Show_End TEXT,
            CharEnd TEXT
        )",
        [],
    )?;

    // Insert data into showdataDB
    let mut rdr = csv::Reader::from_reader(File::open(DATASET2)?);
    for result in rdr.records() {
        let record = result?;
        conn.execute(
            "INSERT INTO showdataDB (Performer, Show, Show_Start, Show_End, CharEnd) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![&record[0], &record[1], &record[2], &record[3], &record[4]],
        )?;
    }

    Ok(())
}
