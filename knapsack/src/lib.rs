use std::collections::BinaryHeap;

#[derive(Debug, PartialEq, Eq)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // (self.value, -(self.weight as i32)).cmp(&(other.value, -(other.weight as i32)))
        ( self.value.abs_diff(self.weight) ).cmp( &other.value.abs_diff(other.weight) )
        
    }
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    let mut weight = 0;
    let mut value = 0;
    let mut heap: BinaryHeap<&Item> = BinaryHeap::from_iter(items);

    println!("{:?}", heap);

    while let Some(item) = heap.pop() {
        if weight + item.weight > max_weight {
            continue
        }

        weight += item.weight;
        value += item.value;

        if weight == max_weight {
            return value
        }
    }
    
    value
}
