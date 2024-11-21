pub fn s_to_double(s: &str) -> f64 {
    if s.is_empty() {
        return 0.0;
    }



    let result : f64 = s.parse().unwrap_or(0.0);

    return result;
}

#[cfg(test)]
mod tests {
    use crate::vputils::s_to_double;

    fn t_s_to_double() {
        assert_eq!(s_to_double("0"), 0.0);
        assert_eq!(s_to_double("5"), 5.0);
        assert_eq!(s_to_double("-5"), -5.0);


    }

}