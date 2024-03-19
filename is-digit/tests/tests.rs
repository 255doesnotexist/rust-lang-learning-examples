#[cfg(test)]
mod tests {
    use is_digit::IsDigit;

    #[test]
    fn char_is_digit() {
        let alpha = 'z';
        let digit = '1';
        let hex_digit = 'F';

        assert!(digit.is_dec_digit());
        assert!(!alpha.is_dec_digit());
        assert!(!hex_digit.is_dec_digit());
    }

    #[test]
    fn str_is_digit() {
        let alpha = "x";
        let digit = "2";
        let hex_digit = "E";

        assert!(digit.is_dec_digit());
        assert!(!alpha.is_dec_digit());
        assert!(!hex_digit.is_dec_digit());
    }

    #[test]
    fn String_is_digit() {
        let alpha = String::from("W");
        let digit = String::from("9");
        let hex_digit = String::from("d");

        assert!(digit.is_dec_digit());
        assert!(!alpha.is_dec_digit());
        assert!(!hex_digit.is_dec_digit());
    }
}