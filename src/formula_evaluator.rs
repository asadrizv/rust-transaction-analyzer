// formula_evaluator.rs

use crate::error::{AppError, FormulaEvaluationErrorEnum};
use crate::csv_parser::{Formula, parse_formula};
use crate::csv_parser::{CsvData};


#[derive(Debug)]
pub struct EvaluatedFormula {
    pub name: String,
    pub value: Option<f64>,
}

#[derive(Debug)]
pub struct EvaluatedRow {
    pub row_number: usize,
    pub formulas: Vec<EvaluatedFormula>,
}

pub fn evaluate_formulas(data: CsvData) -> Result<Vec<EvaluatedRow>, AppError> {
    let mut results: Vec<EvaluatedRow> = Vec::new();

    for row in data.rows {
        let mut evaluated_formulas: Vec<EvaluatedFormula> = Vec::new();
        let mut row_has_error = false;

        for (i, formula_str) in row.formulas.iter().enumerate() {
            let formula = parse_formula(formula_str)?;

            match formula {
                Formula::Literal(value) => {
                    evaluated_formulas.push(EvaluatedFormula {
                        name: format!("{}_{}", row.row_number, i),
                        value: Some(value),
                    });
                }
                Formula::Reference(reference) => {
                    let parts: Vec<&str> = reference.split('@').collect();

                    if parts.len() != 2 {
                        return Err(AppError::FormulaEvaluationError(FormulaEvaluationErrorEnum::InvalidFormula("invalid formula".parse().unwrap())));
                    }

                    let column_label = parts[0];
                    let row_offset = parts[1].parse::<usize>()?;

                    let column_data = data.columns.get(column_label).ok_or(AppError::FormulaEvaluationError);
                    let referenced_formula = column_data?.get(row_offset).ok_or(AppError::FormulaEvaluationError)?;

                    if referenced_formula.value.is_none() {
                        row_has_error = true;
                        break;
                    }

                    evaluated_formulas.push(EvaluatedFormula {
                        name: format!("{}_{}", row.row_number, i),
                        value: referenced_formula.value,
                    });
                }
                _ => {
                    row_has_error = true;
                    break;
                }
            }
        }

        if !row_has_error {
            results.push(EvaluatedRow {
                row_number: row.row_number,
                formulas: evaluated_formulas,
            });
        }
    }

    Ok(results)
}
