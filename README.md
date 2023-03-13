# Rust Transaction Analyzer

A Rust command-line application to analyze transaction data in CSV format.

## Overview

This application takes in CSV files of transaction data and analyzes them to provide useful insights. It supports basic arithmetic expressions, as well as function calls that provide additional features like comparisons, string concatenations, and other useful utility functions.

## Installation

This application requires Rust to be installed on your machine. You can download and install Rust by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

After installing Rust, you can build the application by running the following command in your terminal:

```
cargo build --release

```

## Usage

To run the application, use the following command:

```bash
./target/release/rust-transaction-analyzer /path/to/input.csv /path/to/output.csv

```

Replace `/path/to/input.csv` with the path to your input CSV file and `/path/to/output.csv` with the path where you want to save the output CSV file.

## CSV Format

The CSV files must have the following format:

- The first row must contain the column labels.
- Any computable expression in the CSV must be prefixed with `=`.
- Columns can have labels, which allows the ability to have different column groups in the same file as long as the number of columns stays consistent.
- The following operations are supported:
    - `^^` copies the formula from the cell above in the same column, with some special evaluation rules.
    - `(A..Z)n` references a cell by a combination of a column-letter and row-number. Ex: `A2`, `B3`.
    - `A^` copies the evaluated result of the cell above in the same column.
    - `!label` columns can have labels.
    - `A^v` copies the evaluated result of the last cell in the specified column from the most recently available column group that has data in that specified column.
    - `@label<n>` references a specific labeled column and a specific row n under that column relative to where the column was labeled. This is a reference operator with relative row traversal.

## Output

The output CSV file will have the same format as the input CSV file, but with additional columns containing the calculated results.

