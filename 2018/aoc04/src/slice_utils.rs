pub trait LeftInclusiveSplit {
    type Item;
    fn li_split<F>(&self, predicate: F) -> Vec<Vec<Self::Item>>
        where
            F: FnMut(&Self::Item) -> bool;
}

impl<T: Clone> LeftInclusiveSplit for [T] {
    type Item = T;

    fn li_split<F>(&self, mut predicate: F) -> Vec<Vec<Self::Item>>
        where
            F: FnMut(&Self::Item) -> bool,
    {
        let mut splits = vec![];
        for x in self {
            if (predicate)(x) {
                splits.push(vec![x.clone()]);
            } else {
                match splits.last_mut() {
                    Some(s) => s.push(x.clone()),
                    None => splits.push(vec![x.clone()]),
                }
            }
        }
        splits
    }
}

