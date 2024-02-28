#[derive(Debug, PartialEq, Eq)]
pub struct CustomSet<T> {
    items: Vec<T>
}

impl<T: PartialEq + Clone> CustomSet<T> {
    pub fn new(_input: &[T]) -> Self {
        let mut items: Vec<T> = _input.to_vec();
        items.dedup();

        Self { items }
    }

    pub fn contains(&self, _element: &T) -> bool {
        self.items.contains(_element)
    }

    pub fn add(&mut self, _element: T) {
        if !self.items.contains(&_element) {
            self.items.push(_element);
        }
    }

    pub fn is_subset(&self, _other: &Self) -> bool {
        for item in self.items.iter() {
            if !_other.contains(item) {
                return false
            }
        }
        true
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn is_disjoint(&self, _other: &Self) -> bool {
        for item in self.items.iter() {
            if _other.contains(item) {
                return false
            }
        }
        true
    }

    #[must_use]
    pub fn intersection(&self, _other: &Self) -> Self {
        let mut items: Vec<T> = vec![];

        for item in self.items.iter() {
            if _other.contains(item) {
                items.push(item.clone())
            }
        };

        Self { items }
    }

    #[must_use]
    pub fn difference(&self, _other: &Self) -> Self {
        let items = self.items
            .iter()
            .filter(|&i| !_other.contains(&i))
            .map(|i| i.to_owned())
            .collect();

        Self { items }
    }

    #[must_use]
    pub fn union(&self, _other: &Self) -> Self {
        todo!();
    }
}
