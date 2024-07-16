#![allow(unused)]

type Result<T> = std::result::Result<T, Error>;

#[derive(Default)]
struct Parser {
    lines: Vec<Number>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

#[derive(Clone)]
enum Number {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Unknown,
    Comma,
}

impl Parser {
    fn new() -> Self {
        Self::default()
    }

    fn add_nums(&mut self, input: &[&str]) {
        if !self.lines.is_empty() {
            self.lines.push(Number::Comma)
        }

        (0..input[0].len()).step_by(3).for_each(|i| {
            let block: Number = [
                &input[0].as_bytes()[i..i + 3],
                &input[1].as_bytes()[i..i + 3],
                &input[2].as_bytes()[i..i + 3],
            ]
            .into();

            self.lines.push(block)
        })
    }
}

impl<'a> From<[&'a [u8]; 3]> for Number {
    fn from(value: [&'a [u8]; 3]) -> Self {
        match value[..] {
            [b" _ ", b" _|", b"|_ "] => Number::Two,
            [b" _ ", b" _|", b" _|"] => Number::Three,
            [b" _ ", b"|_ ", b" _|"] => Number::Five,
            [b" _ ", b"|_ ", b"|_|"] => Number::Six,
            [b" _ ", b"  |", b"  |"] => Number::Seven,
            [b" _ ", b"|_|", b"|_|"] => Number::Eight,
            [b" _ ", b"|_|", b" _|"] => Number::Nine,
            [b" _ ", b"| |", b"|_|"] => Number::Zero,
            [_, b"|_|", b"  |"] => Number::Four,
            [_, b"  |", b"  |"] => Number::One,
            _ => Number::Unknown,
        }
    }
}

impl ToString for Parser {
    fn to_string(&self) -> String {
        self.lines.iter().map(|n| n.to_string()).collect()
    }
}

impl ToString for Number {
    fn to_string(&self) -> String {
        let num = self.clone() as u8;

        match num {
            n @ 0..=9 => n.to_string(),
            11 => ",".to_string(),
            _ => "?".to_string(),
        }
    }
}

fn valid_input(inp: &[&str]) -> Result<()> {
    match inp.len() {
        n if n % 4 != 0 || n == 0 => Err(Error::InvalidRowCount(n)),
        _ => match inp[0].len() {
            l if l % 3 != 0 || l == 0 => Err(Error::InvalidColumnCount(l)),
            _ => Ok(()),
        },
    }
}

pub fn convert(input: &str) -> Result<String> {
    let lines: Vec<&str> = input.lines().collect();
    valid_input(&lines)?;

    let mut num_parse = Parser::new();

    (0..lines.len())
        .step_by(4)
        .for_each(|i| num_parse.add_nums(&lines[i..i + 4]));

    Ok(num_parse.to_string())
}
