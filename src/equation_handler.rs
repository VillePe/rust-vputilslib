#![allow(dead_code)]

use std::collections::HashMap;
use crate::equation_handler::FactorType::Operator;

pub const MATH_OPERATORS: &[&str] = &["sqrt", "abs", "sin", "cos", "tan", "acos", "asin", "atan", "log", "log10"];

#[derive(Debug)]
pub struct EquationHandler {
    variables: HashMap<String, f64>,
    formula_string: String,
}

impl EquationHandler {
    pub fn new() -> Self {
        EquationHandler{
            variables: HashMap::new(),
            formula_string: String::new(),
        }
    }

    pub fn add_variable(&mut self, name: &str, value: f64) -> bool {
        let key = String::from(name).to_lowercase();
        if !self.variables.contains_key(&key) {
            self.variables.insert(key, value);
            return true
        }
        false
    }

    pub fn variable_is_set(&self, variable: &str) -> bool {
        self.variables.contains_key(variable)
    }

    pub fn clear_variables(&mut self) {
        self.variables.clear()
    }

    pub fn set_variables(&mut self, variables: HashMap<String, f64>) {
        self.variables.clear();
        for (k, v) in variables {
            let key = String::from(k).to_lowercase();
            self.variables.insert(key, v);
        }
    }

    pub fn get_variable(&self, variable: &str) -> Option<f64> {
        self.variables.get(variable).cloned()
    }

    pub fn set_variable(&mut self, variable: &str, value: f64) {
        let key = String::from(variable).to_lowercase();
        self.variables.insert(key, value);
    }
}

#[derive(Debug)]
pub enum FactorType {
    Number = 1,
    Variable = 2,
    Operator = 3,
    None = 0
}

#[derive(Debug)]
pub struct Factor {
    index: i32,
    length: i32,
    double_value: f64,
    key: String,
    factor_type: FactorType,
}

impl Factor {
    pub fn new(index: i32, length: i32, key: String, factor_type: FactorType) -> Self {
        Factor {
            index, length, double_value: 0.0,
            key, factor_type
        }
    }

    pub fn new_variable(index: i32, length: i32, key: String, double_value: f64) -> Self {
        Factor {
            index, length, double_value,
            key, factor_type: FactorType::Variable
        }
    }

    pub fn new_number(index: i32, length: i32, double_value: f64) -> Self {
        Factor {
            index, length, double_value,
            key: "\0".to_string(), factor_type: FactorType::Number
        }
    }

    pub fn get_operand_value(&self) -> i32 {
        match self.factor_type {
            Operator => {
                if self.key == "(" || self.key == ")" {
                    return 0;
                } else if self.key == "+" || self.key == "-" {
                    return 1;
                } else if self.key == "*" || self.key == "/" {
                    return 2;
                } else {
                    3
                }
            }
            _ => {3}
        }
    }

    pub fn perform_calculation(value: f64, op: Factor, f2: f64) -> f64 {
        match op.factor_type {
            Operator => {
                match op.key.as_str() {
                    "+" => {
                        value + f2
                    }
                    "-" => {
                        value - f2
                    }
                    "*" => {
                        value * f2
                    }
                    "/" => {
                        value / f2
                    }
                    "^" => {
                        value.powf(f2)
                    }
                    _ => {
                        0.0
                    }
                }
            }
            _ => {0.0}
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::equation_handler::{EquationHandler};
    #[test]
    fn equation_handler() {
        let mut equation_handler = EquationHandler::new();
        equation_handler.add_variable("x", 1.0);
        equation_handler.add_variable("y", 1.0);
        assert_eq!(equation_handler.add_variable("x", 1.0), false);

    }
}