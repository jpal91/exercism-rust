use std::collections::HashMap;
use std::thread;

fn counter(inp: &[&str]) -> HashMap<char, usize> {
    let mut map = HashMap::new();

    inp.iter()
        .flat_map(|l| {
            l.chars()
                .filter_map(|c| c.is_alphabetic().then_some(c.to_ascii_lowercase()))
        })
        .for_each(|c| {
            map.entry(c).and_modify(|v| *v += 1).or_insert(1);
        });

    map
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    match input.len() {
        0 => HashMap::new(),
        n if n < 100 => counter(input),
        n => thread::scope(|s| {
            let chunks = input.chunks(n / worker_count + 1);
            let handles = chunks.map(|c| s.spawn(|| counter(c))).collect::<Vec<_>>();
            let mut map = HashMap::new();

            handles.into_iter().for_each(|h| {
                h.join()
                    .unwrap()
                    .into_iter()
                    .for_each(|(k, v)| (*map.entry(k).or_default()) += v)
            });

            map
        }),
    }
}
