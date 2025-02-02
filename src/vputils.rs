use core::str;
use std::ffi::{c_char, CStr};

#[allow(dead_code)]
#[repr(C)]
struct DoubleBoolTuple {
    value: f64,
    is_valid: u8,
}

#[allow(dead_code)]
#[no_mangle]
extern fn s_to_double_extern(s: *const c_char) -> f64 {
    let c_str = unsafe { CStr::from_ptr(s) };
    let s_str = c_str.to_str().unwrap();
    s_to_double_validation(s_str).0.unwrap_or(0.0)
}

#[allow(dead_code)]
#[no_mangle]
extern fn s_to_double_validation_extern(s: *const c_char) -> DoubleBoolTuple {
    let c_str = unsafe { CStr::from_ptr(s) };
    let tuple = s_to_double_validation(c_str.to_str().unwrap());
    let valid : u8 = if tuple.1 {1} else {0};
    DoubleBoolTuple{value: tuple.0.unwrap_or(0.0), is_valid: valid}
}

/// Parses the given string and converts it to double value. String can contain any number of
/// invalid characters but the parsing will be done by dropping the invalid characters.
/// Valid characters are:
/// - digits \[0-9]
/// - comma , (is converted to dot)
/// - dot .
/// - dash \- (only before any other valid characters. Note, abc-efg0123 is converted to -123)
///
/// Returns None if string is empty or parsing fails even after dropping the invalid characters
pub fn s_to_double(s: &str) -> Option<f64> {
    s_to_double_validation(s).0
}

/// Parses the given string and converts it to double value. String can contain any number of
/// invalid characters but the parsing will be done by dropping the invalid characters.
/// Valid characters are:
/// - digits \[0-9]
/// - comma , (is converted to dot)
/// - dot .
/// - dash \- (only before any other valid characters. Note, abc-efg0123 is converted to -123)
///
/// Returns a tuple of Option<f64> and bool
/// - First item (Option<f64>): None if string is empty or parsing fails even after dropping the invalid characters
/// - Second item (bool): is set to false if any invalid character is found or if dash is at the wrong location
pub fn s_to_double_validation(s: &str) -> (Option<f64>, bool) {
    let mut valid_value = true;
    if s.is_empty() {
        return (None, false);
    }

    // Allocate the string that will be converted to double with known max value
    let mut num_value : String = String::with_capacity(s.len());

    let chars = s.char_indices();

    for i in chars {
        if !i.1.is_ascii_digit() && i.1 != ',' && i.1 != '.' && i.1 != '-' {
            // If there are any value that is not any of the characters declared in 'if'
            // invalidate the result (if user needs the value to be valid)
            valid_value = false;
            continue;
        }

        // If the negative sign is anywhere else than the start of the value that is parsed
        // invalidate the result (if user needs the value to be valid)
        if i.1 == '-' && !num_value.is_empty() {
            valid_value = false;
            continue;
        }

        if i.1 == ',' {
            // Change commas to dots
            num_value.push('.');
        } else {
            num_value.push(i.1);
        }
    }

    match num_value.parse::<f64>() {
        Ok(result) => (Some(result), valid_value),
        Err(err) => {
            println!("Error! {err}");
            println!("Tried to parse: \"{num_value}\"");
            (None, false) }
    }
}

/// Parses the given string and converts it to integer value. String can contain any number of
/// invalid characters but the parsing will be done by dropping the invalid characters.
/// Valid characters are:
/// - digits \[0-9].
/// - dash \- (only before any other valid characters. Note, abc-efg0123 is converted to -123).
/// - Note! Comma and dot are **not** valid characters. They're dropped so 5.0 is converted to 50. 
/// Use [`s_to_double_validation`] for floating point values.
///
/// Returns None if string is empty or parsing fails even after dropping the invalid characters
pub fn s_to_int(s: &str) -> Option<isize> {
    s_to_int_validation(s).0
}

/// Parses the given string and converts it to integer value. String can contain any number of
/// invalid characters but the parsing will be done by dropping the invalid characters.
/// Valid characters are:
/// - digits \[0-9].
/// - dash \- (only before any other valid characters. Note, abc-efg0123 is converted to -123).
/// - Note! Comma and dot are **not** valid characters. They're dropped so 5.0 is converted to 50. 
/// Use [`s_to_double_validation`] for floating point values.
///
/// Returns a tuple of Option<isize> and bool
/// - First item (Option<isize>): None if string is empty or parsing fails even after dropping the invalid characters
/// - Second item (bool): is set to false if any invalid character is found or if dash is at the wrong location
pub fn s_to_int_validation(s: &str) -> (Option<isize>, bool) {
    let mut valid_value = true;
    if s.is_empty() {
        return (None, false);
    }

    // Allocate the string that will be converted to double with known max value
    let mut num_value : String = String::with_capacity(s.len());

    let chars = s.char_indices();

    for i in chars {
        if !i.1.is_ascii_digit() && i.1 != '-' {
            // If there are any value that is not any of the characters declared in 'if'
            // invalidate the result (if user needs the value to be valid)
            valid_value = false;
            continue;
        }

        // If the negative sign is anywhere else than the start of the value that is parsed
        // invalidate the result (if user needs the value to be valid)
        if i.1 == '-' && !num_value.is_empty() {
            valid_value = false;
            continue;
        }

        num_value.push(i.1);
    }

    match num_value.parse::<isize>() {
        Ok(result) => (Some(result), valid_value),
        Err(err) => {
            println!("Error! {err}");
            println!("Tried to parse: \"{num_value}\"");
            (None, false) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_s_to_double() {
        assert_eq!(s_to_double(""), None);
        assert_eq!(s_to_double("abc"), None);
        assert_eq!(s_to_double("abc-efg"), None);
        assert_eq!(s_to_double("abc-efg,."), None);
        assert_eq!(s_to_double("0"), Some(0.0));
        assert_eq!(s_to_double("5"), Some(5.0));
        assert_eq!(s_to_double("-5"), Some(-5.0));
        assert_eq!(s_to_double("-5,0"), Some(-5.0));
        assert_eq!(s_to_double("-5.0"), Some(-5.0));
        assert_eq!(s_to_double("-.5"), Some(-0.5));
        assert_eq!(s_to_double("-,5"), Some(-0.5));
        assert_eq!(s_to_double("abc-5"), Some(-5.0));
        assert_eq!(s_to_double("你es-5"), Some(-5.0));
        assert_eq!(s_to_double("abc-efg0123"), Some(-123f64));

        let mut is_valid = s_to_double_validation("-5").1;
        assert_eq!(is_valid, true);
        is_valid = s_to_double_validation("你es-5").1;
        assert_eq!(is_valid, false);
    }

    #[test]
    fn t_s_to_int() {
        assert_eq!(s_to_int(""), None);
        assert_eq!(s_to_int("abc"), None);
        assert_eq!(s_to_int("abc-efg"), None);
        assert_eq!(s_to_int("abc-efg,."), None);
        assert_eq!(s_to_int("0"), Some(0));
        assert_eq!(s_to_int("5"), Some(5));
        assert_eq!(s_to_int("-5"), Some(-5));
        assert_eq!(s_to_int("-5,0"), Some(-50));
        assert_eq!(s_to_int("-5.0"), Some(-50));
        assert_eq!(s_to_int("-.5"), Some(-5));
        assert_eq!(s_to_int("-,5"), Some(-5));
        assert_eq!(s_to_int("abc-5"), Some(-5));
        assert_eq!(s_to_int("你es-5"), Some(-5));
        assert_eq!(s_to_int("abc-efg0123"), Some(-123));

        let mut is_valid = s_to_int_validation("-5").1;
        assert_eq!(is_valid, true);
        is_valid = s_to_int_validation("你es-5").1;
        assert_eq!(is_valid, false);
    }

}