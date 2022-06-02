use crate::BTreeList;

/// An iterator over items in a [`BTreeList`].
#[derive(Debug)]
pub struct Iter<'a, T, const B: usize> {
    pub(crate) inner: &'a BTreeList<T, B>,
    pub(crate) index: usize,
    pub(crate) index_back: usize,
}

impl<'a, T, const B: usize> Iterator for Iter<'a, T, B> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.index_back {
            self.index += 1;
            self.inner.get(self.index - 1)
        } else {
            None
        }
    }
}

impl<'a, T, const B: usize> DoubleEndedIterator for Iter<'a, T, B> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.index < self.index_back {
            self.index_back -= 1;
            self.inner.get(self.index_back)
        } else {
            None
        }
    }
}
