use crate::BTreeList;

/// An iterator over items in a [`BTreeList`].
#[derive(Debug)]
pub struct OwnedIter<T> {
    pub(crate) inner: BTreeList<T>,
}

impl<T> Iterator for OwnedIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.inner.is_empty() {
            self.inner.pop_front()
        } else {
            None
        }
    }
}

impl<T> DoubleEndedIterator for OwnedIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if !self.inner.is_empty() {
            self.inner.pop_back()
        } else {
            None
        }
    }
}
