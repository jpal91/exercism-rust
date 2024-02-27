use std::collections::HashMap;
// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.
#[allow(clippy::new_without_default)]
pub struct School {
    roster: HashMap<u32, Vec<String>>
}

impl School {
    pub fn new() -> School {
        Self {
            roster: HashMap::new()
        }
    }

    pub fn add(&mut self, grade: u32, student: &'static str) {
        let entry = self.roster.entry(grade).or_default();
        let item = student.to_string();
        let pos = entry.binary_search(&item).unwrap_or_else(|e| e);

        entry.insert(pos, item);
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut res: Vec<u32> = self.roster.keys().map(|k| k.to_owned()).collect();
        res.sort();
        res
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        if let Some(g) = self.roster.get(&grade) {
            g.to_owned()
        } else {
            Vec::new()
        }
    }
}
