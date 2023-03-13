mod csv_parser;
mod formula_evaluator;
mod error;

use crate::formula_evaluator::evaluate_formulas;
use crate::csv_parser::parse_csv_file;

fn main() {
    let filename = "filename.csv";

    // Parse CSV file
    let data_result = parse_csv_file(filename);
    let data = match data_result {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error parsing CSV file: {}", err);
            return;
        }
    };

    // Evaluate formulas
    let results_result = evaluate_formulas(data);
    let results = match results_result {
        Ok(results) => results,
        Err(err) => {
            eprintln!("Error evaluating formulas: {}", err);
            return;
        }
    };

    // Print results
    for row in results {
        for formula in row.formulas {
            println!("{}: {:?}", formula.name, formula.value);
        }
    }
}
