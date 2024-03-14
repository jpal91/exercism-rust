#![allow(unused)]
use std::boxed::Box;
use std::collections::VecDeque;
use std::ops::Rem;

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
///

pub struct Matcher<T>(Box<dyn Fn(T) -> bool>, String);

impl<T> Matcher<T>
where
    T: PartialEq + Rem<Output = T> + Copy + From<u8> + ToString,
{
    pub fn new<F, S>(_matcher: F, _subs: S) -> Matcher<T>
    where
        F: Fn(T) -> bool + 'static,
        S: ToString,
    {
        let func = Box::new(_matcher);

        Matcher(func, _subs.to_string())
    }

    fn execute(&self, item: &T) -> Option<String> {
        self.0(*item).then_some(self.1.clone())
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
pub struct Fizzy<T> {
    rules: Vec<Matcher<T>>,
}

impl<T> Fizzy<T>
where
    T: PartialEq + Rem<Output = T> + Copy + From<u8> + ToString,
{
    pub fn new() -> Self {
        Fizzy { rules: vec![] }
    }

    // feel free to change the signature to `mut self` if you like
    #[must_use]
    pub fn add_matcher(mut self, _matcher: Matcher<T>) -> Self {
        self.rules.push(_matcher);
        self
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<'a>(
        &'a self,
        _iter: impl Iterator<Item = T> + 'a,
    ) -> impl Iterator<Item = String> + 'a {
        // todo!() doesn't actually work, here; () is not an Iterator
        // that said, this is probably not the actual implementation you desire
        _iter.map(|i| self.run_rules(&i))
    }

    fn run_rules(&self, item: &T) -> String {
        if let Some(s) = self.custom_rules(item) {
            return s;
        }

        match item {
            i if *i % T::from(3) == T::from(0) && *i % T::from(5) == T::from(0) => {
                "fizzbuzz".to_string()
            }
            i if *i % T::from(3) == T::from(0) => "fizz".to_string(),
            i if *i % T::from(5) == T::from(0) => "buzz".to_string(),
            _ => item.to_string(),
        }
    }

    fn custom_rules(&self, item: &T) -> Option<String> {
        let res = self
            .rules
            .iter()
            .filter_map(|r| r.execute(item))
            .collect::<Vec<String>>()
            .join("");

        (!res.is_empty()).then_some(res)
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: PartialEq + Rem<Output = T> + Copy + From<u8> + ToString,
{
    Fizzy::new()
}
