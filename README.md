[![Rust_CI](https://github.com/nogibjj/Rishika_Randev_Individual_2/actions/workflows/cicd_rust.yml/badge.svg)](https://github.com/nogibjj/Rishika_Randev_Individual_2/actions/workflows/cicd_rust.yml)

[![Python_CI](https://github.com/nogibjj/Rishika_Randev_Individual_2/actions/workflows/cicd_py.yml/badge.svg)](https://github.com/nogibjj/Rishika_Randev_Individual_2/actions/workflows/cicd_py.yml)

# Rishika Randev's Individual Project 2/ Mini Project 8 

## ☑️ Requirements:
* Rust source code: The code should comprehensively understand Rust's syntax and unique features.
* Use of LLM: In your README, explain how you utilized an LLM in your coding process.
* SQLite Database: Include a SQLite database and demonstrate CRUD (Create, Read, Update, Delete) operations.
* Optimized Rust Binary: Include a process that generates an optimized Rust binary as a Gitlab Actions artifact that can be downloaded.
* README.md: A file that clearly explains what the project does, its dependencies, how to run the program, and how Gitlab Copilot was used.
* Github/Gitlab Actions: A workflow file that tests, builds, and lints your Rust code.
* Performance comparison report: highlight improvements in speed and memory between the Rust and Python scripts.
* Video Demo: XX

## ☑️ LLM Usage:
I used chatGPT to help convert existing Python code for ETL and querying operations into Rust, and also to help me understand the syntax and characteristics of Rust since it is substantially different from Python.

## ☑️ The Dataset
The dataset used in this project shows diet, physical activity, and nutrition data from the behaviorial risk factors survey across the US in 2023. It is published by the U.S. Department of Health & Human Services and freely available through data.gov at this link: https://catalog.data.gov/dataset/nutrition-physical-activity-and-obesity-behavioral-risk-factor-surveillance-system.

## ☑️ Steps
1. After installing Rust, set up a new Rust project and install dependencies using the following commands:

```bash
cargo new <project>

cargo add csv
cargo add rusqlite
cargo add reqwest
cargo add clap --features derive
```

2. Within the new directory that gets automatically created for the Rust project, a src directory will get created with a lib.rs and main.rs file. lib.rs is where the functions for extracting, loading, and querying the dataset.

<img width="249" alt="Screenshot 2024-10-28 at 12 09 51 PM" src="https://github.com/user-attachments/assets/ed550221-3731-4938-8a4a-2c99df34533b">

* **extract** pulls the dataset csv from the CDC website, and saves a subset of it (first 10 rows and selected columns) to 'data/Behaviors.csv' within the project directory.
* **load** reads the data from 'data/Behaviors.csv' and loads it to a SQLite database called Behavior.db using the rusqlite package.
* **query** allows any general query input to be run against Behavior.db.
  
main.rs uses the Rust package clap to create a CLI to expose the lib.rs functions (as demonstrated in the screenshots at the end of the README). I also created a tests directory in the project with a test.rs file to validate that the extract, load, and query functions are performing as expected.

3. To compile the code, the following commands can be run (they all have slightly different functionalities depending on what kind of build you are looking for):

```bash
cargo check
cargo build
cargo build --release
```
After running the last command, an optimized binary gets stored to your <project>/target/release/<project> directory, and this is the directory from where the binary artifact should get uploaded during the CI/CD process (so you can update your GH Actions yml file accordingly).

4. In order to use your CLI directly with ```<project> <COMMAND>``` instead of typing out ```cargo run``` you need to add the path of the binary file to your PATH variable. For example, I added this line to my ~/.zshrc file:
```bash
export PATH="$PATH:/Users/rishikarandev/Documents/IDS706/Rishika_Randev_Individual_2/rust_sqllite/target/release"
```


5. The screnshots below demonstrate the use of the command line for running CRUD operations against the Behaviors table in Rust:


ETL:

<img width="591" alt="Screenshot 2024-10-28 at 10 44 37 AM" src="https://github.com/user-attachments/assets/870f269d-469c-43a0-bb45-a60f0d0a4cf5">


<img width="545" alt="Screenshot 2024-10-28 at 11 01 43 AM" src="https://github.com/user-attachments/assets/ea3ccf0e-4d61-4ec2-85ec-74f2112d45cd">

Read & Delete:
<img width="1137" alt="Screenshot 2024-10-28 at 11 02 04 AM" src="https://github.com/user-attachments/assets/4d8c3151-d6ba-44d6-8948-8fdf5ea1d4f7">

Create:
<img width="1129" alt="Screenshot 2024-10-28 at 11 04 16 AM" src="https://github.com/user-attachments/assets/6f2467ee-e80e-4638-a79f-aa1f418cf066">

Update:
<img width="1133" alt="Screenshot 2024-10-28 at 11 05 10 AM" src="https://github.com/user-attachments/assets/49690bdc-0721-48d1-8476-4386ff3950c0">



