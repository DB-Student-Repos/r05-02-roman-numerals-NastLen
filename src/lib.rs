use std::fmt::{Display, Formatter, Result};

pub struct Roman{
    value: u32,
}

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut num = self.value;
        let mut result = String::new(); // we create an empty string to store the result (method ::new)
        let roman_numerals = vec![
            ("M", 1000),
            ("CM", 900),
            ("D", 500),
            ("CD", 400),
            ("C", 100),
            ("XC", 90),
            ("L", 50),
            ("XL", 40),
            ("X", 10),
            ("IX", 9),
            ("V", 5),
            ("IV", 4),
            ("I", 1),
            ];
        for &(number, value) in roman_numerals.iter() {
            while num >= value {
                result += number;
                num -= value;
            }
        }

        write!(f, "{}", result) // we write the result to the formatter
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
       Roman {value: num}
    }
}
