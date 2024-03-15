#![allow(unused)]

const PRICE: f32 = 800.0;

struct BookOrder(Vec<u32>);


// impl BookOrder {
//     fn lowest_price(mut self) {

//     }
// }


impl<'a> FromIterator<&'a u32> for BookOrder {
    fn from_iter<T: IntoIterator<Item = &'a u32>>(iter: T) -> Self {
        let mut order = vec![0; 5];
        let mut iter = iter.into_iter();

        while let Some(book) = iter.next() {
            match book {
                1 => order[0] += 1,
                2 => order[1] += 1,
                3 => order[2] += 1,
                4 => order[3] += 1,
                5 => order[4] += 1,
                _ => unreachable!()
            }
        };

        BookOrder(order)
    }
}

impl From<BookOrder> for u32 {
    fn from(value: BookOrder) -> Self {
        let mut order = value.0;
        let mut total = 0.0;

        while !order.iter().all(|o| o == &0) {
            let mut round = 0;

            for i in 0..5 {
                if order[i] == 0 {
                    continue
                }
                order[i] -= 1;
                round += 1;
            };

            match round {
                1 => total += PRICE,
                2 => total += (PRICE * 2.0) * 0.95,
                3 => total += (PRICE * 3.0) * 0.9,
                4 => total += (PRICE * 4.0) * 0.8,
                5 => total += (PRICE * 5.0) * 0.75,
                _ => unreachable!()
            }
        };

        total as u32
        
    }
}

pub fn lowest_price(books: &[u32]) -> u32 {
    BookOrder::from_iter(books).into()
}