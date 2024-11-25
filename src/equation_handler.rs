#![allow(dead_code)]

use crate::equation_handler::FactorType::Operator;
use std::collections::HashMap;

pub const MATH_OPERATORS: &[&str] = &[
    "sqrt", "abs", "sin", "cos", "tan", "acos", "asin", "atan", "log", "log10",
];

#[derive(Debug)]
pub struct EquationHandler {
    variables: HashMap<String, f64>,
    formula_string: String,
    factors: Vec<Factor>,
}

impl EquationHandler {
    pub fn new() -> Self {
        EquationHandler {
            variables: HashMap::new(),
            formula_string: String::new(),
            factors: Vec::new(),
        }
    }

    pub fn add_variable(&mut self, name: &str, value: f64) -> bool {
        let key = String::from(name).to_lowercase();
        if !self.variables.contains_key(&key) {
            self.variables.insert(key, value);
            return true;
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

    pub fn remove_variable(&mut self, variable: &str) {
        self.variables.remove(variable);
    }

    pub fn calculate_formula(&mut self, formula_string: &str) -> Option<f64> {
        self.factors.clear();
        println!("Original: {}", formula_string);

        // TODO Placeholder
        None
    }

    /// Handles initial string formatting for parser. Adds zero prefixes to values starting with
    /// negative sign (dash) and converts E-notation values to use ^ operator
    fn handle_string_formatting(s: &str) -> String {
        println!("Formatting string: {}", s);
        let temp: String;
        // Parser can't handle a string starting with negative sign, but it can handle it if there is a zero in front of the negative sign
        if s.starts_with("-") {
            temp = "0".to_string() + &s;
        } else {
            temp = s.to_string();
        }

        let mut result: String = String::with_capacity(temp.len());
        let mut chars = temp.chars().peekable();
        let mut bracket_count = 0;
        let mut char_index = 0;
        let mut prev_char: char = ' ';
        while let Some(c) = chars.next() {
            let next_char = chars.peek();
            // Parser can't handle values like (-5) or ^-5, but it can handle if there is a zero in
            // front of the negative sign (0-5) and ^0-5
            if c == '^' && next_char == Some(&'-') {
                result.push_str("^(0-");
                let mut closing_bracket_set = false;
                // Skip the '-' character
                chars.next();
                while let Some(c_inner) = chars.clone().next() {
                    // Search for the number value. After no digit or decimal characters are found
                    // add the closing bracket and break the inner loop and continue the outer loop
                    // Maybetodo: Add check for multiple decimal separators
                    if Self::is_number_or_decimal_separator(c_inner) {
                        result.push(c_inner);
                        // Move the original chars iterator to the next char
                        chars.next();
                    } else {
                        // Don't move the original chars iterator so that all characters
                        // are handled with this if else test
                        result.push(')');
                        closing_bracket_set = true;
                        break;
                    }
                }

                // Got to the end of the string and no end parentheses has been set
                // -> Set it and we're done
                if !closing_bracket_set {
                    result.push(')');
                }
            } else if c == '(' && next_char == Some(&'-') {
                result.push_str("(0");
                bracket_count += 1;
            } else if char_index > 0
                && (c == 'E' || c == 'e')
                && Self::is_number_or_decimal_separator(prev_char)
                && (next_char == Some(&'+')
                    || next_char == Some(&'-')
                    || next_char.unwrap().is_ascii_digit())
            {
                // E notation found e.g. 1E+004, 1e04 1e-4
                // Replace E with next line
                result.push_str("*10^(0");
                let mut closing_bracket_set = false;

                // If the char after E is negative or plus sign the jump needs to be 2.
                // Otherwise, the next char must be a number so jump needs to be only 1 char
                if next_char == Some(&'+') || next_char == Some(&'-') {
                    result.push(*next_char.unwrap());
                    chars.next();
                }

                while let Some(c_inner) = chars.clone().next() {
                    // Search for the number value. After no digit or decimal characters are found
                    // add the closing bracket and break the inner loop and continue the outer loop
                    // Maybetodo: Add check for multiple decimal separators
                    if Self::is_number_or_decimal_separator(c_inner) {
                        result.push(c_inner);
                        // Move the original chars iterator to the next char
                        chars.next();
                    } else {
                        // Don't move the original chars iterator so that all characters
                        // are handled with this if else test
                        result.push(')');
                        closing_bracket_set = true;
                        break;
                    }
                }

                // Got to the end of the string and no end parentheses has been set
                // -> Set it and we're done
                if !closing_bracket_set {
                    result.push(')');
                }
            } else if c == ')' {
                bracket_count -= 1;
                result.push(c);
            } else {
                result.push(c);
            }

            prev_char = c;
            char_index += 1;
        }
        if bracket_count > 0 {
            for _ in 0..(bracket_count - 1) {
                result.push(')');
            }
        }

        result
    }

    fn is_number_or_decimal_separator(c: char) -> bool {
        c.is_ascii_digit() || c == '.' || c == ','
    }

    fn is_valid_formula_char(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_' || c == ','
    }
}

#[derive(Debug)]
pub enum FactorType {
    Number = 1,
    Variable = 2,
    Operator = 3,
    None = 0,
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
            index,
            length,
            double_value: 0.0,
            key,
            factor_type,
        }
    }

    pub fn new_variable(index: i32, length: i32, key: String, double_value: f64) -> Self {
        Factor {
            index,
            length,
            double_value,
            key,
            factor_type: FactorType::Variable,
        }
    }

    pub fn new_number(index: i32, length: i32, double_value: f64) -> Self {
        Factor {
            index,
            length,
            double_value,
            key: "\0".to_string(),
            factor_type: FactorType::Number,
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
            _ => 3,
        }
    }

    pub fn perform_calculation(value: f64, op: Factor, f2: f64) -> f64 {
        match op.factor_type {
            Operator => match op.key.as_str() {
                "+" => value + f2,
                "-" => value - f2,
                "*" => value * f2,
                "/" => value / f2,
                "^" => value.powf(f2),
                _ => 0.0,
            },
            _ => 0.0,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::equation_handler::EquationHandler;

    #[test]
    fn equation_handler() {
        let mut equation_handler = EquationHandler::new();
        equation_handler.add_variable("x", 1.0);
        equation_handler.add_variable("y", 1.0);
        assert_eq!(equation_handler.add_variable("x", 1.0), false);
    }

    #[test]
    fn string_formatter() {
        assert_eq!(
            EquationHandler::handle_string_formatting("5+5*15"),
            "5+5*15"
        );
        assert_eq!(
            EquationHandler::handle_string_formatting("5+5*15-50"),
            "5+5*15-50"
        );
        assert_eq!(
            EquationHandler::handle_string_formatting("5+5*(15+15)"),
            "5+5*(15+15)"
        );
        assert_eq!(
            EquationHandler::handle_string_formatting("(5+5)*15"),
            "(5+5)*15"
        );
        assert_eq!(
            EquationHandler::handle_string_formatting("-5+5*15"),
            "0-5+5*15"
        );
        assert_eq!(
            EquationHandler::handle_string_formatting("-5+5*15*(-15)*10^-5"),
            "0-5+5*15*(0-15)*10^(0-5)"
        );
        assert_eq!(
            EquationHandler::handle_string_formatting("-5+5*15*(-15)*10^-5^-5"),
            "0-5+5*15*(0-15)*10^(0-5)^(0-5)"
        );
        assert_eq!(
            EquationHandler::handle_string_formatting("-5+5*15*(-15)*10^-5+5"),
            "0-5+5*15*(0-15)*10^(0-5)+5"
        );
        assert_eq!(
            EquationHandler::handle_string_formatting("5+5*(-15)"),
            "5+5*(0-15)"
        );
        assert_eq!(
            EquationHandler::handle_string_formatting("5+5*(-15+5)"),
            "5+5*(0-15+5)"
        );
        assert_eq!(
            EquationHandler::handle_string_formatting("5+5^2*(-15)"),
            "5+5^2*(0-15)"
        );
        assert_eq!(
            EquationHandler::handle_string_formatting("5+5*10^3*(-15)"),
            "5+5*10^3*(0-15)"
        );
        assert_eq!(
            EquationHandler::handle_string_formatting("5+5*(-15)^2"),
            "5+5*(0-15)^2"
        );
        assert_eq!(
            EquationHandler::handle_string_formatting("5/5*(-15)"),
            "5/5*(0-15)"
        );
        assert_eq!(
            EquationHandler::handle_string_formatting("5+(5*(-15))"),
            "5+(5*(0-15))"
        );
        assert_eq!(
            EquationHandler::handle_string_formatting("TESTI*2"),
            "TESTI*2"
        );
        assert_eq!(
            EquationHandler::handle_string_formatting("TESTI*2E5"),
            "TESTI*2*10^(05)"
        );
        assert_eq!(
            EquationHandler::handle_string_formatting("TESTI*2E5+TESTI"),
            "TESTI*2*10^(05)+TESTI"
        );
        assert_eq!(
            EquationHandler::handle_string_formatting("(TESTI*2E5)+TESTI"),
            "(TESTI*2*10^(05))+TESTI"
        );
        assert_eq!(
            EquationHandler::handle_string_formatting("(TESTI*2E+5)+TESTI"),
            "(TESTI*2*10^(0+5))+TESTI"
        );
        assert_eq!(
            EquationHandler::handle_string_formatting("(TESTI*2E-5)+TESTI"),
            "(TESTI*2*10^(0-5))+TESTI"
        );
        assert_eq!(
            EquationHandler::handle_string_formatting("(TESTI*2E-005)+TESTI"),
            "(TESTI*2*10^(0-005))+TESTI"
        );
        assert_eq!(
            EquationHandler::handle_string_formatting("TESTI*(2E5+TESTI)"),
            "TESTI*(2*10^(05)+TESTI)"
        );
        assert_eq!(
            EquationHandler::handle_string_formatting("TESTI*(2E+5+TESTI)"),
            "TESTI*(2*10^(0+5)+TESTI)"
        );
        assert_eq!(
            EquationHandler::handle_string_formatting("TESTI*(2E-5+TESTI)"),
            "TESTI*(2*10^(0-5)+TESTI)"
        );
        assert_eq!(
            EquationHandler::handle_string_formatting("TESTI*(2E-005+TESTI)"),
            "TESTI*(2*10^(0-005)+TESTI)"
        );
        assert_eq!(
            EquationHandler::handle_string_formatting("3,49199E-06"),
            "3,49199*10^(0-06)"
        );
    }
}
