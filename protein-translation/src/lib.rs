use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    // We fake using 'a here, so the compiler does not complain that
    // "parameter `'a` is never used". Delete when no longer needed.
    pair_map: HashMap<&'a str, &'a str>,
}

// enum Codon {
//     AUG,
//     UUU,
//     UUC,
//     UUA,
//     UUG,

// }

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        if let Some(res) = self.pair_map.get(codon) {
            Some(*res)
        } else {
            None
        }
        
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let mut res: Vec<&str> = vec![];

        for chunk in rna.as_bytes().chunks(3) {
            let key = std::str::from_utf8(chunk).unwrap();
            
            if is_stopper(key) {
                return Some(res)
            };
            
            if let Some(val) = self.name_for(key) {
                res.push(val);
            } else {
                return None
            }
        };

        Some(res)
    }
}

fn is_stopper(rna: &str) -> bool {
    match rna {
        "UAA" | "UAG" | "UGA" => true,
        _ => false
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    let pair_map = HashMap::from_iter(pairs);

    CodonsInfo { pair_map }
}
