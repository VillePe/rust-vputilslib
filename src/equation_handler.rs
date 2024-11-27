#![allow(dead_code)]

use crate::vputils;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

pub const MATH_OPERATORS: &[&str] = &[
    "sqrt", "abs", "sin", "cos", "tan", "acos", "asin", "atan", "log", "log10",
];

#[derive(Debug)]
pub struct EquationHandler {
    variables: HashMap<String, f64>,
}

impl EquationHandler {
    pub fn new() -> Self {
        EquationHandler {
            variables: HashMap::new(),
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
        let formatted_formula_string = Self::handle_string_formatting(formula_string);
        let factors = self.populate_lists_streaming(formatted_formula_string.as_str());
        let input: Vec<Factor> = Self::get_prefix_notation(factors);
        self.calculate_prefix_notation(input)
    }

    /// Handles initial string formatting for parser. Adds zero prefixes to values starting with
    /// negative sign (dash) and converts E-notation values to use ^ operator
    fn handle_string_formatting(s: &str) -> String {
        let mut temp: String;
        // Parser can't handle a string starting with negative sign, but it can handle it if there is a zero in front of the negative sign
        if s.starts_with("-") {
            temp = "0".to_string() + &s;
        } else {
            temp = s.to_string();
        }
        // Remove the whitespace
        temp.retain(|c| !c.is_whitespace());

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

    fn populate_lists_streaming(&self, formula_string: &str) -> Vec<Factor> {
        let mut result = Vec::new();

        let mut current_factor_type = FactorType::None;
        let formula_string = formula_string.to_lowercase();
        let mut buffer = String::with_capacity(formula_string.len());
        let mut running_index = 0;
        let mut brackets = false;
        let mut chars = formula_string.chars().peekable();
        let mut peeked_none_found_counter = 0;
        loop {
            let peeked_opt = chars.peek();
            let current;
            match peeked_opt {
                // We have iterated through the string, but buffer is not empty
                // Set the current char to value that isn't valid in any branch
                // of the match with current_factor_type
                None => {
                    current = ' ';
                    // If peeked was None (= iteration at the end of string) and buffer is empty,
                    // break the loop. If buffer still has stuff in it, run one more lap to clean it
                    // up (if possible)
                    if buffer.is_empty() {
                        return result;
                    }
                    peeked_none_found_counter += 1;
                    // Infinite loop safeguard. None should be only found with peek when
                    // buffer is not emptied, but string is already fully iterated
                    if peeked_none_found_counter > 1 {
                        return result;
                    }
                }
                Some(c) => {
                    current = *c;
                    // Everything in brackets are considered to be comments (useful if equation needs
                    // to have units, e.g. 10 [kN] * 5 [m])
                    if current == '[' {
                        brackets = true;
                        continue;
                    } else if current == ']' {
                        brackets = false;
                    }
                    if brackets {
                        continue;
                    }
                }
            }

            match current_factor_type {
                FactorType::Number => {
                    if Self::is_number_or_decimal_separator(current) {
                        // Next is guaranteed to be some in Some(current) = chars.peek();
                        buffer.push(chars.next().unwrap());
                        continue; // Make sure that chars.peek is checked before continuing
                    } else {
                        let buffer_as_double = vputils::s_to_double(buffer.as_str());
                        match buffer_as_double {
                            Some(d) => {
                                result.push(Factor::new_number(
                                    running_index,
                                    buffer.len() as isize,
                                    d,
                                ));
                            }
                            _ => {}
                        }
                        current_factor_type = FactorType::None;
                        running_index += 1;
                    }
                }
                FactorType::Variable => {
                    // A math operator found. (sqrt, cos, sin, ...)
                    if current == '(' {
                        if MATH_OPERATORS.contains(&buffer.as_str()) {
                            chars.next(); // Skip the opening parenthesis
                            let mut open_parenthesis_count = 0;
                            let mut val_buffer = String::new();
                            while let Some(temp_c) = chars.next() {
                                if temp_c == ')' && open_parenthesis_count == 0 {
                                    let val = self.get_value_from_special_math_op(
                                        buffer.as_str(),
                                        val_buffer.as_str(),
                                    );
                                    if val.is_none() {
                                        println!("Value is none!");
                                        break;
                                    }
                                    result.push(Factor::new_number(
                                        running_index,
                                        val_buffer.len() as isize,
                                        val.unwrap(),
                                    ));
                                    current_factor_type = FactorType::None;
                                    running_index += 1;
                                    // The closing parenthesis is ok to be consumed. We only
                                    // want the number from the math operation
                                    break;
                                } else if temp_c == ')' {
                                    open_parenthesis_count -= 1;
                                } else if temp_c == '(' {
                                    open_parenthesis_count += 1;
                                }
                                val_buffer.push(temp_c);
                            }
                        }
                    }
                    if Self::is_valid_formula_char(current) {
                        // Next is guaranteed to be some in Some(current) = chars.peek();
                        buffer.push(chars.next().unwrap());
                        continue; // Make sure that chars.peek is checked before continuing
                    } else {
                        if self.variables.contains_key(buffer.as_str()) {
                            // A variable has been found. Add new factor and set its double
                            // value to the value saved under the variable key
                            result.push(Factor::new_number(
                                running_index,
                                buffer.len() as isize,
                                self.variables[buffer.as_str()],
                            ));
                            running_index += 1;
                        }
                        current_factor_type = FactorType::None;
                    }
                }
                // If the current factor type is none, try to check which factor type the current
                // character should be and continue the parsing
                FactorType::None => {
                    buffer.clear();
                    if Self::is_number_or_decimal_separator(current) {
                        current_factor_type = FactorType::Number;
                    } else if (current).is_ascii_alphabetic() {
                        current_factor_type = FactorType::Variable;
                    } else if Self::is_operator(current) {
                        let next = chars.next().unwrap();
                        result.push(Factor::new(
                            running_index,
                            1,
                            next.to_string(),
                            FactorType::Operator,
                        ));
                        current_factor_type = FactorType::None;
                        running_index += 1;
                    }
                }
                FactorType::Operator => {
                    // Not used in this method!
                }
            }
        }
    }

    fn get_prefix_notation(factors: Vec<Factor>) -> Vec<Factor> {
        let mut operand_stack: Vec<Factor> = Vec::new();
        let mut output_queue: Vec<Factor> = Vec::new();

        for f in factors {
            if f.factor_type == FactorType::Number || f.factor_type == FactorType::Variable {
                output_queue.push(f);
            } else if f.factor_type == FactorType::Operator {
                if f.key == "(" {
                    operand_stack.push(f);
                } else if f.key == ")" {
                    while let Some(stack_f) = operand_stack.pop() {
                        // If '(' is found, pop it and ignore it
                        if stack_f.key == "(" {
                            break;
                        }
                        // Put all operators from operand stack to queue until end of stack or when
                        // opening parenthesis is found => both parenthesis are found
                        output_queue.push(stack_f);
                    }
                } else {
                    // Put all operands that have bigger or equals value compared to current operand
                    // to the output queue
                    while operand_stack.len() > 0 {
                        // Take a peek of the last item (top of the stack)
                        let peek = operand_stack.last();
                        if peek.is_some() {
                            // If the current factors operand value is less or equals to the next in
                            // stack push the next into the output queue
                            // If both operands are '^' characters don't flush the operand stack
                            // because the order of operations is not the same as in dividing and multiplying
                            // e.g. 5^5^5 is the same as (5^(5^5))
                            if f.get_operand_value() <= peek.unwrap().get_operand_value()
                                && !(f.key == "^" && peek.unwrap().key == "^")
                            {
                                let op = operand_stack.pop().unwrap();
                                output_queue.push(op);
                            } else {
                                break;
                            }
                        }
                    }
                    operand_stack.push(f);
                }
            }
        }

        while operand_stack.len() > 0 {
            output_queue.push(operand_stack.pop().unwrap());
        }

        output_queue
    }

    fn calculate_prefix_notation(&self, mut input_queue: Vec<Factor>) -> Option<f64> {
        if input_queue.len() == 1 {
            return Some(input_queue.pop().unwrap().double_value);
        }
        let mut output_stack = Vec::new();
        for current in input_queue {
            if current.factor_type == FactorType::Number
                || current.factor_type == FactorType::Variable
            {
                output_stack.push(current.double_value);
            } else if current.factor_type == FactorType::Operator {
                let pop1 = output_stack.pop().unwrap();
                let pop2 = output_stack.pop().unwrap();
                let temp_calc = Factor::perform_calculation(pop2, current, pop1);
                output_stack.push(temp_calc);
            }
        }

        if output_stack.len() != 1 {
            println!("ERROR with prefix notation calculations!");
            return None;
        }
        Some(output_stack.pop().unwrap())
    }

    fn get_value_from_special_math_op(&self, operation: &str, valuestr: &str) -> Option<f64> {
        // Create an inner equation handler to handle equations inside math operations
        // e.g. sin(alpha-50)
        let mut eq: EquationHandler = EquationHandler::new();
        // Add the variables from current equation handler to temporary inner equation handler
        for v in self.variables.iter() {
            eq.add_variable(v.0, *v.1);
        }
        let value = eq.calculate_formula(valuestr);
        if value.is_none() {
            return None;
        }
        match operation {
            "sqrt" => Some(value.unwrap().sqrt()),
            "abs" => Some(value.unwrap().abs()),
            "sin" => Some(value.unwrap().sin()),
            "cos" => Some(value.unwrap().cos()),
            "tan" => Some(value.unwrap().tan()),
            "acos" => Some(value.unwrap().acos()),
            "asin" => Some(value.unwrap().asin()),
            "atan" => Some(value.unwrap().atan()),
            "log" => Some(value.unwrap().log(std::f64::consts::E)),
            "log10" => Some(value.unwrap().log10()),
            _ => None,
        }
    }

    fn is_number_or_decimal_separator(c: char) -> bool {
        c.is_ascii_digit() || c == '.' || c == ','
    }

    fn is_valid_formula_char(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_' || c == ','
    }

    fn is_operator(c: char) -> bool {
        c == '+' || c == '-' || c == '/' || c == '*' || c == '(' || c == ')' || c == '^'
    }
}

#[derive(Debug, PartialEq)]
pub enum FactorType {
    Number = 1,
    Variable = 2,
    Operator = 3,
    None = 0,
}

pub struct Factor {
    index: isize,
    length: isize,
    double_value: f64,
    key: String,
    factor_type: FactorType,
}

impl Factor {
    pub fn new(index: isize, length: isize, key: String, factor_type: FactorType) -> Self {
        Factor {
            index,
            length,
            double_value: 0.0,
            key,
            factor_type,
        }
    }

    pub fn new_variable(index: isize, length: isize, key: String, double_value: f64) -> Self {
        Factor {
            index,
            length,
            double_value,
            key,
            factor_type: FactorType::Variable,
        }
    }

    pub fn new_number(index: isize, length: isize, double_value: f64) -> Self {
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
            FactorType::Operator => {
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
            FactorType::Operator => match op.key.as_str() {
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

impl Debug for Factor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.factor_type {
            FactorType::Number => {
                write!(f, "{:.1}", self.double_value)
            }
            FactorType::Variable => {
                write!(f, "{} + {:.1}", self.key, self.double_value)
            }
            FactorType::Operator => {
                write!(f, "{:.1}", self.key)
            }
            FactorType::None => {
                write!(f, "None!")
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::equation_handler::EquationHandler;

    #[test]
    fn equation_handler() {
        let mut equation_handler = EquationHandler::new();
        equation_handler.add_variable("x", 1.0);
        equation_handler.add_variable("y", 1.0);
        assert_eq!(equation_handler.add_variable("x", 1.0), false);
        assert_eq!(equation_handler.calculate_formula("x+y"), Some(2.0));
    }

    #[test]
    fn list_population() {
        test_list_population("(TESTI*2E-005)+TESTI", 15);
        test_list_population("sin(50)", 1);
        test_list_population("sin(50*2+100/4+TESTI)", 1);
    }

    fn test_list_population(s: &str, expected: usize) {
        let mut equation_handler = EquationHandler::new();
        equation_handler.add_variable("TESTI", 1.0);
        let factors = equation_handler
            .populate_lists_streaming(EquationHandler::handle_string_formatting(s).as_str());
        assert_eq!(factors.len(), expected);
    }

    #[test]
    fn string_formatter() {
        assert_eq!(
            EquationHandler::handle_string_formatting("5 +5*15"),
            "5+5*15"
        );
        assert_eq!(
            EquationHandler::handle_string_formatting("5+5*15-50"),
            "5+5*15-50"
        );
        assert_eq!(
            EquationHandler::handle_string_formatting("5+5* ( 15+15)"),
            "5+5*(15+15)"
        );
        assert_eq!(
            EquationHandler::handle_string_formatting("( 5+5)  *1 5"),
            "(5+5)*15"
        );
        assert_eq!(
            EquationHandler::handle_string_formatting("-5+5*15"),
            "0-5+5*15"
        );
        assert_eq!(
            EquationHandler::handle_string_formatting("-5+ 5*15 *(-15 )*1 0^ -5"),
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

    #[test]
    fn equations() {
        let mut equation_handler = EquationHandler::new();
        equation_handler.add_variable("TESTI", 10.0);
        assert_eq!(equation_handler.calculate_formula("5 +5*15"), Some(80.0));
        assert_eq!(equation_handler.calculate_formula("5+5*15-50"), Some(30.0));
        assert_eq!(
            equation_handler.calculate_formula("5+5* ( 15+15)"),
            Some(155.0)
        );
        assert_eq!(
            equation_handler.calculate_formula("( 5+5)  *1 5"),
            Some(150.0)
        );
        assert_eq!(equation_handler.calculate_formula("-5+5*15"), Some(70.0));
        assert_eq!(
            equation_handler.calculate_formula("-5+ 5*15 *(-15 )*1 0^ -5"),
            Some(-5.01125)
        );
        assert!(
            (equation_handler
                .calculate_formula("-5+5*15*(-15)*10^-5^-5")
                .unwrap()
                .abs()
                - 1129.1713746)
                < 0.0001
        );
        assert!(
            (equation_handler
                .calculate_formula("-5+5*15*(-15)*10^-5+5")
                .unwrap()
                .abs()
                - 0.01125)
                < 0.01
        );
        assert!(
            (equation_handler
                .calculate_formula("5+5*(-15)")
                .unwrap()
                .abs()
                - 70.0)
                < 0.1
        );
        assert!(
            (equation_handler
                .calculate_formula("5+5*(-15+5)")
                .unwrap()
                .abs()
                - 45.0)
                < 0.1
        );
        assert!(
            (equation_handler
                .calculate_formula("5+5^2*(-15)")
                .unwrap()
                .abs()
                - 370.0)
                < 0.1
        );
        assert!(
            (equation_handler
                .calculate_formula("5+5*10^3*(-15)")
                .unwrap()
                .abs()
                - 74995.0)
                < 0.1
        );
        assert!(
            (equation_handler
                .calculate_formula("5+5*(-15)^2")
                .unwrap()
                .abs()
                - 1130.0)
                < 0.1
        );
        assert!(
            (equation_handler
                .calculate_formula("5/5*(-15)")
                .unwrap()
                .abs()
                - 15.0)
                < 0.1
        );
        assert!(
            (equation_handler
                .calculate_formula("5+(5*(-15))")
                .unwrap()
                .abs()
                - 70.0)
                < 0.1
        );
        assert!((equation_handler.calculate_formula("TESTI*2").unwrap().abs() - 20.0) < 0.1);
        assert!(
            (equation_handler
                .calculate_formula("TESTI*2E5")
                .unwrap()
                .abs()
                - 2000000.0)
                < 0.1
        );
        assert!(
            (equation_handler
                .calculate_formula("TESTI*2E5+TESTI")
                .unwrap()
                .abs()
                - 2000010.0)
                < 0.1
        );
        assert!(
            (equation_handler
                .calculate_formula("(TESTI*2E5)+TESTI")
                .unwrap()
                .abs()
                - 2000010.0)
                < 0.1
        );
        assert!(
            (equation_handler
                .calculate_formula("(TESTI*2E+5)+TESTI")
                .unwrap()
                .abs()
                - 2000010.0)
                < 0.1
        );
        assert!(
            (equation_handler
                .calculate_formula("(TESTI*2E-5)+TESTI")
                .unwrap()
                .abs()
                - 10.0002)
                < 0.0001
        );
        assert!(
            (equation_handler
                .calculate_formula("(TESTI*2E-005)+TESTI")
                .unwrap()
                .abs()
                - 10.0002)
                < 0.0001
        );
        assert!(
            (equation_handler
                .calculate_formula("TESTI*(2E5+TESTI)")
                .unwrap()
                .abs()
                - 2000100.0)
                < 0.1
        );
        assert!(
            (equation_handler
                .calculate_formula("TESTI*(2E+5+TESTI)")
                .unwrap()
                .abs()
                - 2000100.0)
                < 0.1
        );
        assert!(
            (equation_handler
                .calculate_formula("TESTI*(2E-5+TESTI)")
                .unwrap()
                .abs()
                - 100.0002)
                < 0.00001
        );
        assert!(
            (equation_handler
                .calculate_formula("TESTI*(2E-005+TESTI)")
                .unwrap()
                .abs()
                - 100.0002)
                < 0.00001
        );
        assert!(
            (equation_handler
                .calculate_formula("3,49199E-06")
                .unwrap()
                .abs()
                - 0.00000349199)
                < 0.00001
        );
    }
}
