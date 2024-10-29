use std::fs::File;
use std::fs;
use std::io::Write;
use rusqlite::{params, Connection, Result};
use reqwest::blocking::Client;
use std::time::Instant;
use std::fs::OpenOptions;
use sysinfo::System;


pub fn extract() -> Result<String, Box<dyn std::error::Error>> {
    if !fs::metadata("data").is_ok() {
        fs::create_dir_all("data").expect("Failed to create directory");
    }

    // Download the data & save to file
    let url = "https://data.cdc.gov/api/views/hn4x-zwk7/rows.csv?accessType=DOWNLOAD";
    let client = Client::new();
    let mut response = client.get(url).send().expect("Failed to send request");
    let file_path = "data/Behaviors.csv";
    let full_path = "data/Behaviors_full.csv";
    let mut file = File::create(full_path).expect("Failed to create file");
    std::io::copy(&mut response, &mut file).expect("Failed to copy content");
    
    // Read the CSV file
    let mut rdr = csv::Reader::from_path(full_path)?;
    let mut wtr = csv::Writer::from_path(file_path)?;

    // Write the subset of data
    for (i, result) in rdr.records().enumerate() {
        if i >= 10 {
            break; // only take first 10 records
        }
        let record = result?;
        wtr.write_record(&[
            &record[0], // YearStart
            &record[1], // YearEnd
            &record[2], // LocationAbbr
            &record[3], // LocationDesc
            &record[7], // Question
            &record[10], // Data_Value
        ])?;
    }
    
    wtr.flush()?;
    println!("Successfully extracted data");

    Ok(file_path.to_string())
}


pub fn load(dataset: &str) -> Result<String> {

    let start_time = Instant::now();
    let sys = System::new_all();
    let initial_memory = sys.used_memory(); // in kilobytes

    // Connect to SQLite database
    let conn = Connection::open("Behavior.db")?;
    
    // Create a new table
    conn.execute("DROP TABLE IF EXISTS Behaviors", [])?;
    conn.execute(
        "CREATE TABLE Behaviors (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            YearStart INTEGER,
            YearEnd INTEGER,
            LocationAbbr TEXT,
            LocationDesc TEXT,
            Question TEXT,
            Data_Value INTEGER
        )",
        [],
    )?;

    // Open the CSV file
    let mut rdr = csv::ReaderBuilder::new().has_headers(false).from_path(dataset).expect("Failed to read dataset");
    
    // Prepare the insert statement
    let mut stmt = conn.prepare(
        "INSERT INTO Behaviors (YearStart, YearEnd, LocationAbbr, LocationDesc, Question, Data_Value)
        VALUES (?, ?, ?, ?, ?, ?)"
    )?;

    // Insert each record into the database
    for result in rdr.records() {
        match result {
            Ok(record) => {
                stmt.execute(&[
                    &record[0], &record[1], &record[2], &record[3], &record[4], &record[5],
                ])?;
            }
            Err(err) => {
                eprintln!("Error reading CSV record: {:?}", err);
            }
        }
    }
    println!("Successfully transformed and loaded data to SQLite");
    let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time).as_secs_f64();

    let final_memory = sys.used_memory();
    let memory_usage = final_memory - initial_memory;

    println!("Rust-Elapsed Time: {:.7} seconds", elapsed_time);
    println!("Rust-Memory Usage Change: {} kilobytes", memory_usage);

    Ok("Behavior.db".to_string())
}

pub fn query(query: &str) -> Result<()> {
    let conn = Connection::open("Behavior.db")?;
    // Read operation
    if query.trim().to_lowercase().starts_with("select") {
        let mut stmt = conn.prepare(query)?;
        let results = stmt.query_map(params![], |row| {
            Ok((
                row.get::<usize, i32>(0)?,
                row.get::<usize, i32>(1)?,
                row.get::<usize, i32>(2)?,
                row.get::<usize, String>(3)?,
                row.get::<usize, String>(4)?,
                row.get::<usize, String>(5)?,
                row.get::<usize, f32>(6)?,
            ))
        })?;

        for result in results {
            match result {
                Ok((
                    id,
                    YearStart, 
                    YearEnd, 
                    LocationAbbr, 
                    LocationDesc, 
                    Question, 
                    Data_Value
                )) => {
                    println!(
                        "Result: id={}, YearStart={}, YearEnd={}, LocationAbbr={}, LocationDesc={}, Question={}, Data_Value={}",
                        id, YearStart, YearEnd, LocationAbbr,
                        LocationDesc, Question, Data_Value
                    );
                }
                Err(e) => eprintln!("Error in row: {:?}", e),
            }
        }
    } else {
        // other CUD operations
        let _num_affected_rows = conn.execute_batch(query)?;
    }
    Ok(())
}