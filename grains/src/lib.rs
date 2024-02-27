pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64")
    }
    
    let mut res: u64 = 1;
    for _ in 1..s {
        res *= 2;
    }
    res
}

pub fn total() -> u64 {
    // let mut res: u64 = 0;
    // for i in 0..64 {
    //     res += square(i + 1);
    // }
    // res
    18_446_744_073_709_551_615
}
