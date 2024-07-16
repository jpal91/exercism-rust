#![allow(unused)]

fn diamonds(ord: u8, n: usize, mut v: Vec<String>) -> Vec<String> {
    let mut new_str = vec![' '; n];
    let mid = n / 2;
    new_str[mid + ord as usize] = (ord + b'A') as char;
    new_str[mid - ord as usize] = (ord + b'A') as char;

    v.push(new_str.into_iter().collect());

    if ord == 0 {
        return v
    };

    diamonds(ord - 1, n, v)
}

pub fn get_diamond(c: char) -> Vec<String> {
    let ord = (c as u8) - b'A';
    let n = ((2 * ord) + 1) as usize;

    let bottom = diamonds(ord, n, vec![]);
    bottom
        .clone()
        .into_iter()
        .rev()
        .take(ord as usize)
        .chain(bottom)
        .collect()
}
