

pub fn is_armstrong_number(num: u64) -> bool {
    let str_num: String = num.to_string();
    let len: u32 = str_num.len() as u32;
    let mut res:u64 = 0;

    for n in str_num.chars() {
        let p_num: u64 = n.to_digit(10).unwrap() as u64;
        res += p_num.pow(len)
    };
    println!("{}", res);
    res == num
}
