pub trait LeftInclusiveSplit {
    type Item;
    fn split(&self) -> Vec<Vec<Self::Item>>;
}

impl<T> LeftInclusiveSplit for [T] {
    type Item = T;

    fn split(&self) -> Vec<Vec<Self::Item>> {
        unimplemented!()
    }
}