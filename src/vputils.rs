use core::str;

pub fn s_to_double(s: &str) -> Option<f64> {
    s_to_double_validation(s, &mut false)
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
pub fn s_to_double_validation(s: &str, mut _valid_value: &mut bool) -> Option<f64> {
    *_valid_value = true;
    if s.is_empty() {
        return None;
    }

    // Allocate the string that will be converted to double with known max value
    let mut num_value : String = String::with_capacity(s.len());

    let chars = s.char_indices();

    for i in chars {
        print!("{0:?}", i.1);
        if !i.1.is_ascii_digit() && i.1 != ',' && i.1 != '.' && i.1 != '-' {
            // If there are any value that is not any of the characters declared in 'if'
            // invalidate the result (if user needs the value to be valid)
            *_valid_value = false;
            continue;
        }

        // If the negative sign is anywhere else than the start of the value that is parsed
        // invalidate the result (if user needs the value to be valid)
        if i.1 == '-' && !num_value.is_empty() {
            *_valid_value = false;
            continue;
        }

        if i.1 == ',' {
            // Change commas to dots
            num_value.push('.');
        } else {
            num_value.push(i.1);
        }
    }

    println!();
    println!("Num value: {0}", num_value.len());

    match num_value.parse() {
        Ok(result) => {
            Some(result) },
        Err(err) => {
            println!("Error! {err}");
            println!("Tried to parse: {num_value}");
            None }
    }
}

#[cfg(test)]
mod tests {
    use crate::vputils::{s_to_double, s_to_double_validation};

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

        let mut is_valid = false;
        assert_eq!(s_to_double_validation("-5", &mut is_valid), Some(-5.0));
        assert_eq!(is_valid, true);
        assert_eq!(s_to_double_validation("你es-5", &mut is_valid), Some(-5.0));
        assert_eq!(is_valid, false);
    }

}