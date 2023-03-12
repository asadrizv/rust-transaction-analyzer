use std::fmt;
use std::num::ParseIntError;

// Define custom error types
#[derive(Debug)]
pub enum CsvParserError {
    IoError(std::io::Error),
    CsvError(csv::Error),
}

impl fmt::Display for CsvParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CsvParserError::IoError(err) => write!(f, "I/O error: {}", err),
            CsvParserError::CsvError(err) => write!(f, "CSV error: {}", err),
        }
    }
}

impl std::error::Error for CsvParserError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CsvParserError::IoError(err) => Some(err),
            CsvParserError::CsvError(err) => Some(err),
        }
    }
}

#[derive(Debug)]
pub enum FormulaEvaluationErrorEnum {
    InvalidFormula(String),
    VariableNotFound(String),
    InvalidOperation(String),
}

impl fmt::Display for FormulaEvaluationErrorEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FormulaEvaluationErrorEnum::InvalidFormula(var) => {
                write!(f, "Invalid formula: {}", var)
            }
            FormulaEvaluationErrorEnum::VariableNotFound(var) => {
                write!(f, "Variable not found: {}", var)
            }
            FormulaEvaluationErrorEnum::InvalidOperation(op) => {
                write!(f, "Invalid operation: {}", op)
            }
        }
    }
}

impl std::error::Error for FormulaEvaluationErrorEnum {}

#[derive(Debug)]
pub enum AppError {
    CsvParsingError(CsvParserError),
    FormulaEvaluationError(FormulaEvaluationErrorEnum),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::CsvParsingError(err) => write!(f, "CSV parsing error: {}", err),
            AppError::FormulaEvaluationError(err) => {
                write!(f, "Formula evaluation error: {}", err)
            }
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::CsvParsingError(err) => Some(err),
            AppError::FormulaEvaluationError(err) => Some(err),
        }
    }
}

impl From<std::io::Error> for CsvParserError {
    fn from(err: std::io::Error) -> Self {
        CsvParserError::IoError(err)
    }
}

impl From<CsvParserError> for AppError {
    fn from(error: CsvParserError) -> Self {
        AppError::CsvParsingError(error)
    }
}

impl From<FormulaEvaluationErrorEnum> for AppError {
    fn from(error: FormulaEvaluationErrorEnum) -> Self {
        AppError::FormulaEvaluationError(error)
    }
}

impl From<ParseIntError> for AppError {
    fn from(error: ParseIntError) -> Self {
        AppError::FormulaEvaluationError(FormulaEvaluationErrorEnum::InvalidFormula(format!("Error parsing integer: {}", error)))
    }
}