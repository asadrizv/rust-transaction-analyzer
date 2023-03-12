// calculator.rs

use crate::csv_parser::CsvData;
use crate::error::{AppError, FormulaEvaluationErrorEnum};
use crate::error::AppError::FormulaEvaluationError;
use crate::formula_evaluator::{EvaluatedRow};
use std::collections::HashMap;

// Define function to calculate adjusted cost
pub fn calculate_adjusted_cost(data: &CsvData) -> Vec<f64> {
    let mut adjusted_cost = vec![0.0; data.rows.len()];
    adjusted_cost[0] = data.rows[0]
        .formulas[0]
        .parse::<f64>()
        .unwrap_or(0.0);
    for i in 1..data.rows.len() {
        let total_cost = data.rows[i]
            .formulas[0]
            .parse::<f64>()
            .unwrap_or(0.0);
        adjusted_cost[i] = total_cost + (data.fee * adjusted_cost[i - 1]);
    }
    adjusted_cost
}
// Define function to check if cost is too high
pub fn is_cost_too_high(adjusted_cost: &Vec<f64>, cost_threshold: f64) -> bool {
    for i in 0..adjusted_cost.len() {
        if adjusted_cost[i] > cost_threshold {
            return true;
        }
    }
    false
}

pub fn calculate_results(results: Vec<EvaluatedRow>) -> Result<HashMap<String, f64>, AppError> {
    let mut final_results: HashMap<String, f64> = HashMap::new();

    for row in results {
        for formula in row.formulas {
            let formula_name = formula.name.clone();
            let formula_value = formula.value.ok_or(FormulaEvaluationError(FormulaEvaluationErrorEnum::InvalidFormula("err".parse().unwrap())))?;

            let entry = final_results.entry(formula_name).or_insert(0.0);
            *entry += formula_value;
        }
    }

    Ok(final_results)
}