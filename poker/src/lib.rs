#![allow(unused)]
/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.

mod checks;

#[derive(PartialEq, Eq, Clone, Hash)]
enum CardType {
    Ace,
    King,
    Queen,
    Jack,
    Num(u8)
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum Suit {
    Spade = 1,
    Diamond = 2,
    Club = 3,
    Heart = 4
}

enum HandType {
    Flush,
    FourOK,
    FullHouse,
    ThreeOK,
    Pair,
    HC
}

#[derive(PartialEq, Eq, Clone, Hash)]
struct Card(CardType, Suit);

struct Hand {
    cards: Vec<Card>
}

impl Hand {
    fn new(cards: &str) -> Self {
        let cards: Vec<Card> = cards
            .split(" ")
            .map(|c| Card::new(c))
            .collect();

        Self { cards }
    }
}

impl Card {
    fn new(card: &str) -> Self {
        let card = card.as_bytes();
        let n = card.len();

        Card(
            CardType::new(&card[..n-1]),
            Suit::new(&card[n-1])
        )
    }

    fn val(&self) -> u8 {
        self.0.val() + (self.1 as u8)
    }
}

impl Suit {
    fn new(suit: &u8) -> Self {
        match suit {
            b'D' => Self::Diamond,
            b'S' => Self::Spade,
            b'C' => Self::Club,
            b'H' => Self::Heart,
            _ => unimplemented!()
        }
    }
}

impl CardType {
    fn new(ctype: &[u8]) -> Self {
        match ctype {
            [b'A'] => Self::Ace,
            [b'K'] => Self::King,
            [b'Q'] => Self::Queen,
            [b'J'] => Self::Jack,
            b"10" => Self::Num(10),
            c => Self::Num(u8::from(c[0]))
        }
    }

    fn val(&self) -> u8 {
        match self {
            CardType::Ace => 14,
            CardType::King => 13,
            CardType::Queen => 12,
            CardType::Jack => 11,
            CardType::Num(n) => *n
        }
    }
}

impl HandType {

}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.val().cmp(&other.val())
    }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    if hands.len() == 1 {
        return hands.into()
    };

    vec![]
}
