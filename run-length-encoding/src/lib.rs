pub fn encode(source: &str) -> String {
    if source.is_empty() {
        return String::new()
    }

    let mut current: Option<char> = None;
    let mut count = 0;
    let mut res = String::new();

    for c in source.chars() {
        if current.is_none() {
            current = Some(c);
        }

        if current != Some(c) {
            if count > 1 {
                res += &format!("{}{}", count, current.unwrap());
            } else {
                res.push(current.unwrap())
            }
            current = Some(c);
            count = 0;
        };

        count += 1;
        
    };

    if count > 1 {
        res += &format!("{}{}", count, current.unwrap());
    } else {
        res.push(current.unwrap())
    }
    println!("{}", res);
    res
}

pub fn decode(source: &str) -> String {
    if source.is_empty() {
        return String::new()
    }

    let mut count = String::new();
    let mut res = String::new();

    for c in source.chars() {
        if c.is_ascii_digit() {
            count.push(c)
        } else {
            let n: u32 = count.parse().unwrap_or(1);
            (0..n).for_each(|_| res.push(c));
            count.clear();
        } 
    };

    res
}
