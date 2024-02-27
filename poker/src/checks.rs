use crate::{Hand, Card};
use counter::Counter;

fn is_flush(hand: &Hand) -> bool {
    let mut hand = hand.cards.clone();
    hand.sort();
    
    for i in 0..4 {
        if &hand[i].val() != &hand[i + 1].val() {
            return false
        }
    }

    true
}

fn get_counter(hand: &Hand) -> Counter<Card> {
    hand.cards.clone().into_iter().collect()
}

fn multis(count: &Counter<Card>) -> Option<u8> {
    let common = count.k_most_common_ordered(1);
    match &common[0].1 {
        &4 => Some(4),
        &3 => Some(3),
        &2 => Some(2),
        _ => None
    }
}

fn is_full_house(count: &Counter<Card>) -> bool {
    let common = count.k_most_common_ordered(2);

    if &common[0].1 != &3 && &common[1].1 != &2 {
        false
    } else {
        true
    }

}