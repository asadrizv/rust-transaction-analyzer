mod csv_parser;
mod formula_evaluator;
mod error;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use formula_evaluator::evaluate_formulas;
use csv_parser::parse_csv_file;
use toml::Value;

fn main() -> Result<(), Box<dyn Error>> {
    // Read config file
    let mut config_file = File::open("config.toml")?;
    let mut config_str = String::new();
    config_file.read_to_string(&mut config_str)?;
    let config: Value = toml::from_str(&config_str)?;

    let filename = config["filename"].as_str().unwrap();

    // Parse CSV file
    let data = parse_csv_file(filename)?;

    println!("{:#?}", data);

    // Evaluate formulas
    let results = evaluate_formulas(data)?;

    // Write results to output CSV file
    let path = Path::new("output.csv");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    if results.is_empty(){
        println!("No results to write to output.csv");
    }

    for row in &results {
        let mut row_values = vec![];

        for formula in &row.formulas {
            if let Some(value) = formula.value {
                row_values.push(value.to_string());
            } else {
                row_values.push(String::new());
            }
        }

        let row_string = row_values.join(",");
        file.write_all(row_string.as_bytes())?;
        file.write_all(b"\n")?;
    }

    println!("Results written to output.csv");

    Ok(())
}
