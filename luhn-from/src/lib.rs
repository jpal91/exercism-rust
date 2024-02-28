pub struct Luhn(bool);

impl Luhn {
    pub fn is_valid(&self) -> bool {
        self.0
    }
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?

impl<T> From<T> for Luhn 
where
    T: ToString
{
    fn from(input: T) -> Self {
        let input = input.to_string();

        if input.trim().len() <= 1 {
            return Luhn(false)
        }

        if let Some(sum) = luhn_strs(&input) {
            Luhn(sum % 10 == 0)
        } else {
            Luhn(false)
        }
    }
}


fn luhn_strs(input: &str) -> Option<u32> {
    let mut nums: Vec<u32> = vec![];
    let code = input.chars().filter(|c| c != &' ').rev().enumerate();

    for (i, c) in code {
        if !c.is_ascii_digit() {
            return None;
        };

        let num = match c.to_digit(10) {
            Some(n) if i % 2 != 0 && n * 2 > 9 => (n * 2) - 9,
            Some(n) if i % 2 != 0 => n * 2,
            Some(n) => n,
            None => return None,
        };

        nums.push(num)
    }

    Some(nums.into_iter().sum::<u32>())
}