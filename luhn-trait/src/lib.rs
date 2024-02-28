pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T> Luhn for T 
where
    T: ToString
{
    fn valid_luhn(&self) -> bool {
        let input = self.to_string();

        if input.trim().len() <= 1 {
            return false
        }

        let mut nums: Vec<u32> = vec![];
        let code = input.chars().filter(|c| c != &' ').rev().enumerate();
    
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
}
