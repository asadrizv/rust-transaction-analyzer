use std::collections::HashMap;
use std::fs::File;

use csv;

use crate::error::{AppError, CsvParserError, FormulaEvaluationErrorEnum};
use crate::error::AppError::FormulaEvaluationError;
use crate::formula_evaluator::EvaluatedFormula;

#[derive(Debug)]
pub struct CsvData {
    pub rows: Vec<CsvRow>,
    pub columns: HashMap<String, Vec<EvaluatedFormula>>,
    pub fee: f64,
    pub cost_threshold: f64,
}

#[derive(Debug)]
pub struct CsvRow {
    pub row_number: usize,
    pub formulas: Vec<String>,
}

pub enum Formula {
    Literal(f64),
    Reference(String),
}

pub fn parse_formula(formula_str: &str) -> Result<Formula, AppError> {
    if let Ok(value) = formula_str.parse::<f64>() {
        return Ok(Formula::Literal(value));
    }

    if let Some('@') = formula_str.chars().find(|c| !c.is_numeric()) {
        return Ok(Formula::Reference(formula_str.to_string()));
    }

    Err(FormulaEvaluationError(
        FormulaEvaluationErrorEnum::InvalidFormula(format!(
            "Invalid formula: {}",
            formula_str
        )),
    ))
}

pub fn parse_csv_file(filename: &str) -> Result<CsvData, AppError> {
    let file = File::open(filename).map_err(|err| AppError::CsvParsingError(CsvParserError::IoError(err)))?;
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .trim(csv::Trim::All)
        .from_reader(file);

    let mut rows = vec![];
    let mut columns = HashMap::new();
    let mut fee = 0.0;
    let mut cost_threshold = 0.0;
    let mut prev_fields: Option<usize> = None;

    for (i, result) in reader.records().enumerate() {
        let record = match result {
            Ok(r) => r,
            Err(_) => {
                // Skip over records that couldn't be parsed
                continue;
            }
        };

        if record[0].starts_with('!') {
            if record[0].to_string() == "!fee" {
                fee = record[1]
                    .parse::<f64>()
                    .map_err(|err| AppError::FormulaEvaluationError(FormulaEvaluationErrorEnum::InvalidFormula(format!("Error parsing float: {}", err))))?;
            } else if record[0].to_string() == "!cost_threshold" {
                cost_threshold = record[1]
                    .parse::<f64>()
                    .map_err(|err| AppError::FormulaEvaluationError(FormulaEvaluationErrorEnum::InvalidFormula(format!("Error parsing float: {}", err))))?;
            }
        } else {
            let mut formulas: Vec<String> = vec![];
            let mut tokens: Vec<String> = vec![];

            if record.len() < 3 {
                continue;
            }

            if let Some(pf) = prev_fields {
                if record.len() != pf {
                    // Skip over records with different number of fields than the previous record
                    continue;
                }
            }

            for (j, cell) in record[2].split(',').enumerate() {
                let column_label = format!("{}_{}", j + 1, record[0].to_string());

                if cell.starts_with('=') {
                    formulas.push(cell[1..].to_string());
                    tokens.push(column_label.clone());
                } else {
                    let value = cell.parse::<f64>().ok();

                    columns
                        .entry(column_label.clone())
                        .or_insert_with(|| vec![])
                        .push(EvaluatedFormula {
                            name: format!("{}_value", column_label),
                            value,
                        });
                }
            }

            rows.push(CsvRow {
                row_number: i,
                formulas,
            });

            for token in tokens {
                columns
                    .entry(token.clone())
                    .or_insert_with(|| vec![])
                    .push(EvaluatedFormula {
                        name: format!("{}_price", token),
                        value: None,
                    });
            }

            prev_fields = Some(record.len());
        }
    }

    Ok(CsvData {
        rows,
        columns,
        fee,
        cost_threshold,
    })
}

