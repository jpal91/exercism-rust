pub fn build_proverb(list: &[&str]) -> String {
    let len: usize = list.len();
    let mut res: String = String::new();

    if len == 0 {
        return res
    }

    for i in 1..len {
        res.push_str(&format!("For want of a {} the {} was lost.\n", list[i - 1], list[i]))
        
    };

    res.push_str(&format!("And all for the want of a {}.", list[0]));
    res
}
