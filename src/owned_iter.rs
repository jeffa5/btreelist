use crate::BTreeList;

/// An iterator over items in a [`BTreeList`].
#[derive(Debug)]
pub struct OwnedIter<T, const B: usize> {
    pub(crate) inner: BTreeList<T, B>,
}

impl<T, const B: usize> Iterator for OwnedIter<T, B> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.inner.is_empty() {
            self.inner.pop_front()
        } else {
            None
        }
    }
}

impl<T, const B: usize> DoubleEndedIterator for OwnedIter<T, B> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if !self.inner.is_empty() {
            self.inner.pop_back()
        } else {
            None
        }
    }
}
