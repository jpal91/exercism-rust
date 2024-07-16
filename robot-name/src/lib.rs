
#[derive(Debug)]
pub struct Robot(String);

impl Robot {
    pub fn new() -> Self {
        Robot(gen_new_name())
    }

    pub fn name(&self) -> &str {
        &self.0
    }

    pub fn reset_name(&mut self) {
        self.0 = gen_new_name();
    }
}

// This appears to be the sweet spot. Actual nanos looped through the
// combinations too fast causing repeats in the mapped test.
fn time() -> u128 {
    std::time::UNIX_EPOCH.elapsed().unwrap().as_nanos() / 100
}

// More deterministic than random, but seems random in a relatively small data set
// (ie < ~670,000) without having to keep global state.
//
// Of the 676,000 combinations, uses modulo of system time to determine the order we are in
// the list of combos. 
fn gen_new_name() -> String {
    let mut name = String::new();
    let t = time() % 676000;
    let letters = (t / 1000) % 676;
    let numbers = t % 1000;

    name.push(char::from_u32((letters / 26) as u32 + 65).unwrap());
    name.push(char::from_u32((letters % 26) as u32 + 65).unwrap());

    if numbers < 100 {
        name.push('0')
    }

    if numbers < 10 {
        name.push('0')
    }

    name += &numbers.to_string();

    name
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        for _ in 0..10 {
            println!("{:?}", gen_new_name())
        }
    }
}