/// Checks if the specified character is an ASCII digit.
///
/// This method verifies whether the character's ASCII value falls within the range of '0' (ASCII value 48) to '9' (ASCII value 57).
/// If so, it returns `true`, indicating that it is a digit. Otherwise, it returns `false`.
///
/// # Examples
///
/// ```
/// use is_digit::IsDigit;
///
/// let digit = '1';
/// assert!(digit.is_dec_digit());
///
/// let alpha = 'a';
/// assert!(!alpha.is_dec_digit());
/// ```
/// 
pub trait IsDigit {
    fn is_dec_digit(&self) -> bool;
}

macro_rules! prim_impl {
    ($($t:ty)*) => {
        $(
            impl IsDigit for $t {
                fn is_dec_digit(&self) -> bool {
                    *self >= '0' as $t && *self <= '9' as $t
                }
            }
        )*
    };
}

prim_impl!(char);

macro_rules! adapt_impl {
    ($($t:ty)*) => {
        $(
            impl IsDigit for $t {
                fn is_dec_digit(&self) -> bool {
                    self.as_char().unwrap().is_dec_digit()
                }
            }
        )*
    };
}

adapt_impl!(&str String);

pub trait AsChar {
    fn as_char(&self) -> Option<char>;
}

impl AsChar for &str {
    fn as_char(&self) -> Option<char> {
        if self.len() == 1 {
            self.chars().next()
        } else {
            None
        }
    }
}

impl AsChar for String {
    fn as_char(&self) -> Option<char> {
        if self.len() == 1 {
            self.chars().next()
        } else {
            None
        }
    }
}