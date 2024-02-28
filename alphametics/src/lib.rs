
#![allow(unused)]
use std::collections::HashMap;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut split_eq = input.split("==");

    if split_eq.clone().count() != 2 {
        return None;
    };

    let operand = parse_operand(split_eq.next().unwrap());
    let value = str_to_u32(split_eq.next().unwrap());
    todo!()
}

fn parse_operand(input: &str) -> u32 {
    let it = input.split("+");
    let mut total = 0;

    for v in it {
        let num = str_to_u32(v);
        total += num
    };

    total
}

fn str_to_u32(input: &str) -> u32 {
    let val: String = input
    .trim()
    .as_bytes()
    .iter()
    .map(|&x| char::from_u32((x - 65) as u32).unwrap())
    .collect();

    val.parse().unwrap()
} 