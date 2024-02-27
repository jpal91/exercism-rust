use std::fmt;

pub fn verse(n: i32) -> String {
    Beer(n).to_string()
}

pub fn sing(start: i32, end: i32) -> String {
    let mut res: String = String::new();

    for i in (end..=start).rev() {
        res.push_str(&verse(i));
        res += "\n"
    }
    res.pop();
    res
}

struct Beer(i32);

impl Beer {
    fn pt1(&self, num: i32) -> String {
        match num {
            num if num > 1 => String::from(format!("{} bottles of beer on the wall, {} bottles of beer.\n", num, num)),
            num if num == 1 => String::from("1 bottle of beer on the wall, 1 bottle of beer.\n"),
            _ => String::from("No more bottles of beer on the wall, no more bottles of beer.\n")
        }
    }

    fn pt2(&self, num: i32) -> String {
        match num {
            num if num > 1 => String::from(format!("Take one down and pass it around, {} bottles of beer on the wall.\n", num)),
            num if num == 1 => String::from("Take one down and pass it around, 1 bottle of beer on the wall.\n"),
            num if num == 0 => String::from("Take it down and pass it around, no more bottles of beer on the wall.\n"),
            _ => String::from("Go to the store and buy some more, 99 bottles of beer on the wall.\n")
        }
    }
}

impl fmt::Display for Beer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let n: i32 = self.0;
        let p1: String = self.pt1(n);
        let p2: String = self.pt2(n - 1);

        write!(f, "{}{}", p1, p2)
    }
}