//this will be the CLI portion of the project where we accept
//user defined arguments and call lib.rs logic to handle them
use clap::{Parser, Subcommand};
use rusqlite::{Connection, Result};
use rust_sqllite::{extract, load, query}; //import library logic

//Here we define a struct (or object) to hold our CLI arguments
//for #[STUFF HERE] syntax, these are called attributes. Dont worry about them
//for now, they define behavior for elements in rust.

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
//Think of a struct as a class which makes objects in python
//This is designed to generate an object out of the CLI inputs
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

//An enum is a type in rust that can have multiple exauhstive and mutually exclusive options
//We also know that the command can be 1 of 4 (really 3) options
//Create, Read and Update (query), Delete

#[derive(Debug, Subcommand)]
//By separating out the commands as enum types we can easily match what the user is
//trying to do in main
enum Commands {
    ///Pass a query string to execute Read operations
    #[command(alias = "q", short_flag = 'q')]
    Query { query_string: String },
    ///Pass the path to a dataset that you want to load to SQLite
    #[command(alias = "l", short_flag = 'l')]
    Load {
        dataset: String,
    },
    ///Save the Behaviors dataset to data/Behaviors.csv locally
    #[command(alias = "e", short_flag = 'e')]
    Extract {
    },
}

fn main() -> Result<()> {
    //Here we parse the CLI arguments and store them in the args object
    let args = Cli::parse();
    //generate connection
    let conn = Connection::open("Behavior.db")?;

    //Here we can match the behavior on the subcommand and call our lib logic
    match args.command {
        Commands::Extract {  } => {
            extract().expect("Failed to extract data");
        }
        Commands::Query { query_string } => {
            println!("Query: {}", query_string);
            query(&query_string).expect("Failed to execute query");
        }
        Commands::Load {
            dataset,
        } => {
            println!(
                "Loading data into SQLite from '{}'",
                dataset
            );
            load(&dataset)
                .expect("Failed to load data from csv");
        }
    }
    Ok(())
}
