use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct SeriesIndex<T: Clone + Debug + Ord> {
    pub label: T,
    pub key: usize,
}

impl<T: Clone + Debug + Ord> SeriesIndex<T> {
    pub fn new(label: T, key: usize) -> SeriesIndex<T> {
        SeriesIndex { label, key }
    }
}
