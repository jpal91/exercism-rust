/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code.trim().len() <= 1 {
        return false;
    }

    let mut nums: Vec<u32> = vec![];
    let code = code.chars().filter(|c| c != &' ').rev().enumerate();

    for (i, c) in code {
        if !c.is_ascii_digit() {
            return false;
        };

        let num = match c.to_digit(10) {
            Some(n) if i % 2 != 0 && n * 2 > 9 => (n * 2) - 9,
            Some(n) if i % 2 != 0 => n * 2,
            Some(n) => n,
            None => return false,
        };

        nums.push(num)
    }

    nums.into_iter().sum::<u32>() % 10 == 0
}
