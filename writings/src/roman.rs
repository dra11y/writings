//! Conversion between integers and roman numerals.
//! "Borrowed" from roman crate.

const ROMAN: &[(char, u32)] = &[
    ('I', 1),
    ('V', 5),
    ('X', 10),
    ('L', 50),
    ('C', 100),
    ('D', 500),
    ('M', 1000),
];

const ROMAN_PAIRS: &[(&str, u32)] = &[
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

/// The largest number representable as a roman numeral.
pub const MAX: u32 = 3999;

/// Converts an integer into a roman numeral.
///
/// Works for integer between 1 and 3999 inclusive, returns None otherwise.
pub fn to(n: u32) -> Option<String> {
    if n == 0 || n > MAX {
        return None;
    }
    let mut out = String::new();
    let mut n = n;
    for &(name, value) in ROMAN_PAIRS.iter() {
        while n >= value {
            n -= value;
            out.push_str(name);
        }
    }
    assert!(n == 0);
    Some(out)
}

/// Converts a roman numeral to an integer.
///
/// Works for integer between 1 and 3999 inclusive, returns None otherwise.
#[allow(unused)]
pub fn from(txt: &str) -> Option<u32> {
    let (mut n, mut max) = (0, 0);
    for c in txt.chars().rev() {
        let &(_, val) = ROMAN.iter().find(|&(ch, _)| *ch == c)?;
        if val < max {
            n -= val;
        } else {
            n += val;
            max = val;
        }
    }
    Some(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_roman() {
        let roman =
            "I II III IV V VI VII VIII IX X XI XII XIII XIV XV XVI XVII XVIII XIX XX XXI XXII"
                .split(' ');
        for (i, x) in roman.enumerate() {
            let n = (i + 1) as u32;
            assert_eq!(to(n).unwrap(), x);
        }
        assert_eq!(to(1979).unwrap(), "MCMLXXIX");
        assert_eq!(to(1984).unwrap(), "MCMLXXXIV");
        assert_eq!(to(2024).unwrap(), "MMXXIV");
    }

    #[test]
    fn test_from() {
        assert!(from("i").is_none());
    }

    #[test]
    fn test_to_from() {
        for n in 1..MAX {
            assert_eq!(from(&to(n).unwrap()).unwrap(), n);
        }
    }
}
