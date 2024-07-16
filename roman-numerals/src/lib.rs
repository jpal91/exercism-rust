use std::fmt::{Display, Formatter, Result};

pub struct Roman(Vec<u8>);

impl Display for Roman {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result {
        let res: String = self.0.iter()
            .map(|l| *l as char)
            .collect();

        write!(_f, "{}", res)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        let mut num = num;
        let mut roman: Vec<u8> = vec![];

        while num > 0 {
            let (sub, letters): (u32, &[u8]) = match num {
                n if n >= 1000 => (1000, b"M"),
                n if n >= 900 => (900, b"CM"),
                n if n >= 500 => (500, b"D"),
                n if n >= 400 => (400, b"CD"),
                n if n >= 100 => (100, b"C"),
                n if n >= 90 => (90, b"XC"),
                n if n >= 50 => (50, b"L"),
                n if n >= 40 => (40, b"XL"),
                n if n >= 10 => (10, b"X"),
                n if n >= 9 => (9, b"IX"),
                n if n >= 5 => (5, b"V"),
                n if n == 4 => (4, b"IV"),
                _ => (1, b"I")
            };

            num -= sub;
            roman.extend(letters);
        }
        
        Roman(roman)
    }
}
