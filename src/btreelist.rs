use std::{
    cmp::{min, Ordering},
    iter::FromIterator,
    mem,
    ops::{Index, IndexMut},
};

use crate::{Iter, OwnedIter};

/// A list with efficient insert and removal in the middle.
///
/// It may be worth benchmarking your use case and trying to use a [`Box<T>`](Box) instead of a plain `T`
/// as this can improve performance in some cases.
/// Similar word-length wrapper types would also work e.g. [`Rc`](std::rc::Rc).
///
/// ```
/// # use btreelist::BTreeList;
/// # use btreelist::btreelist;
/// let mut list = BTreeList::default();
/// list.push(1);
/// list.push(2);
///
/// assert_eq!(list.len(), 2);
/// assert_eq!(list[0], 1);
///
/// assert_eq!(list.pop(), Some(2));
/// assert_eq!(list.len(), 1);
///
/// list[0] = 7;
/// assert_eq!(list[0], 7);
///
/// list.extend([1,2,3]);
///
/// for x in &list {
///     println!("{x}");
/// }
/// assert_eq!(list, btreelist![7, 1, 2, 3]);
/// ```
#[derive(Clone, Debug)]
pub struct BTreeList<T, const B: usize = 6> {
    root_node: Option<BTreeListNode<T, B>>,
}

#[derive(Clone, Debug, PartialEq)]
struct BTreeListNode<T, const B: usize> {
    elements: Vec<T>,
    children: Vec<BTreeListNode<T, B>>,
    length: usize,
}

impl<T, const B: usize> BTreeList<T, B> {
    /// Construct a new, empty [`BTreeList`].
    ///
    /// No allocation occurs until elements are added.
    ///
    /// ```
    /// # use btreelist::BTreeList;
    /// // create a BTreeList with the default B parameter
    /// let mut list : BTreeList<i32> = BTreeList::new();
    /// // create a BTreeList with a custom B parameter
    /// let mut list : BTreeList<i32, 32> = BTreeList::new();
    /// ```
    pub fn new() -> Self {
        Self { root_node: None }
    }

    /// Get the length of the list.
    ///
    /// ```
    /// # use btreelist::btreelist;
    /// let list = btreelist![1, 2, 3];
    /// assert_eq!(list.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.root_node.as_ref().map_or(0, |n| n.len())
    }

    /// Check if the list is empty.
    ///
    /// ```
    /// # use btreelist::btreelist;
    /// # use btreelist::BTreeList;
    /// let mut list: BTreeList<_> = BTreeList::new();
    /// assert!(list.is_empty());
    ///
    /// list.push(1);
    /// assert!(!list.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Create an iterator through the list.
    ///
    /// ```
    /// # use btreelist::btreelist;
    /// let x = btreelist![1, 2, 4];
    /// let mut iterator = x.iter();
    ///
    /// assert_eq!(iterator.next(), Some(&1));
    /// assert_eq!(iterator.next(), Some(&2));
    /// assert_eq!(iterator.next(), Some(&4));
    /// assert_eq!(iterator.next(), None);
    /// ```
    pub fn iter(&self) -> Iter<'_, T, B> {
        Iter {
            inner: self,
            index: 0,
            index_back: self.len(),
        }
    }

    /// Insert the `element` into the list at `index`. Returns the element to be inserted if the
    /// index is out of bounds.
    ///
    /// ```
    /// # use btreelist::btreelist;
    /// let mut list = btreelist![1, 2, 3];
    /// list.insert(1, 4);
    /// assert_eq!(list, btreelist![1, 4, 2, 3]);
    /// list.insert(4, 5);
    /// assert_eq!(list, btreelist![1, 4, 2, 3, 5]);
    /// ```
    pub fn insert(&mut self, index: usize, element: T) -> Result<(), T> {
        let old_len = self.len();
        if index > old_len {
            return Err(element);
        }

        if let Some(root) = self.root_node.as_mut() {
            #[cfg(debug_assertions)]
            root.check();

            if root.is_full() {
                let original_len = root.len();
                let new_root = BTreeListNode::new();

                // move new_root to root position
                let old_root = mem::replace(root, new_root);

                root.length += old_root.len();
                root.children.push(old_root);
                root.split_child(0);

                assert_eq!(original_len, root.len());

                // after splitting the root has one element and two children, find which child the
                // index is in
                let first_child_len = root.children[0].len();
                let (child, insertion_index) = if first_child_len < index {
                    (&mut root.children[1], index - (first_child_len + 1))
                } else {
                    (&mut root.children[0], index)
                };
                root.length += 1;
                child.insert_into_non_full_node(insertion_index, element)?
            } else {
                root.insert_into_non_full_node(index, element)?
            }
        } else if index == 0 {
            self.root_node = Some(BTreeListNode {
                elements: vec![element],
                children: Vec::new(),
                length: 1,
            });
        } else {
            return Err(element);
        }
        assert_eq!(self.len(), old_len + 1);
        Ok(())
    }

    /// Push the `element` onto the back of the list.
    ///
    /// ```
    /// # use btreelist::btreelist;
    /// let mut list = btreelist![1, 2];
    /// list.push(3);
    /// assert_eq!(list, btreelist![1, 2, 3]);
    /// ```
    pub fn push(&mut self, element: T) {
        self.push_back(element)
    }

    /// Push the `element` onto the back of the list.
    ///
    /// ```
    /// # use btreelist::btreelist;
    /// let mut list = btreelist![1, 2];
    /// list.push_back(3);
    /// assert_eq!(list, btreelist![1, 2, 3]);
    /// ```
    pub fn push_back(&mut self, element: T) {
        let l = self.len();
        // SAFETY: can always push onto the end of a list
        let _ = self.insert(l, element);
    }

    /// Push the `element` onto the front of the list.
    ///
    /// ```
    /// # use btreelist::btreelist;
    /// let mut list = btreelist![2, 3];
    /// list.push_front(1);
    /// assert_eq!(list, btreelist![1, 2, 3]);
    /// ```
    pub fn push_front(&mut self, element: T) {
        // SAFETY: can always push onto the start of a list
        let _ = self.insert(0, element);
    }

    /// Remove and return the last element from the list, if there is one.
    ///
    /// ```
    /// # use btreelist::btreelist;
    /// let mut list = btreelist![1, 2, 3];
    /// assert_eq!(list.pop(), Some(3));
    /// assert_eq!(list, btreelist![1, 2]);
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        self.pop_back()
    }

    /// Remove and return the last element from the list, if there is one.
    ///
    /// ```
    /// # use btreelist::btreelist;
    /// let mut list = btreelist![1, 2, 3];
    /// assert_eq!(list.pop_back(), Some(3));
    /// assert_eq!(list, btreelist![1, 2]);
    /// ```
    pub fn pop_back(&mut self) -> Option<T> {
        if !self.is_empty() {
            self.remove(self.len() - 1)
        } else {
            None
        }
    }

    /// Remove and return the first element from the list, if there is one.
    ///
    /// ```
    /// # use btreelist::btreelist;
    /// let mut list = btreelist![1, 2, 3];
    /// assert_eq!(list.pop_front(), Some(1));
    /// assert_eq!(list, btreelist![2, 3]);
    /// ```
    pub fn pop_front(&mut self) -> Option<T> {
        if !self.is_empty() {
            self.remove(0)
        } else {
            None
        }
    }

    /// Removes the element at `index` from the list if it exists.
    ///
    /// ```
    /// # use btreelist::btreelist;
    /// let mut list = btreelist![1, 2, 3];
    /// assert_eq!(list.remove(1), Some(2));
    /// assert_eq!(list, btreelist![1, 3]);
    /// ```
    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.len() {
            return None;
        }
        if let Some(root) = self.root_node.as_mut() {
            #[cfg(debug_assertions)]
            let len = root.check();
            let old = root.remove(index)?;

            if root.elements.is_empty() {
                if root.is_leaf() {
                    self.root_node = None;
                } else {
                    self.root_node = Some(root.children.remove(0));
                }
            }

            #[cfg(debug_assertions)]
            debug_assert_eq!(len, self.root_node.as_ref().map_or(0, |r| r.check()) + 1);
            Some(old)
        } else {
            None
        }
    }

    /// Update the `element` at `index` in the list, returning the old value on success, or the
    /// given value when the index is out of bounds.
    ///
    /// ```
    /// # use btreelist::btreelist;
    /// let mut list = btreelist![1, 2, 3];
    /// list.set(1, 4);
    /// assert_eq!(list, btreelist![1, 4, 3]);
    /// ```
    pub fn set(&mut self, index: usize, element: T) -> Result<T, T> {
        if let Some(node) = self.root_node.as_mut() {
            node.set(index, element)
        } else {
            Err(element)
        }
    }

    /// Returns whether the swap was successful.
    pub fn swap(&mut self, a: usize, b: usize) -> bool {
        if a > b {
            self.swap_inner(b, a)
        } else {
            self.swap_inner(a, b)
        }
    }

    /// Swap two elements, assumes `a <= b`.
    /// Returns whether the swap was successful.
    fn swap_inner(&mut self, a: usize, b: usize) -> bool {
        assert!(a <= b);

        let b_elt = match self.remove(b) {
            Some(elt) => elt,
            None => return false,
        };
        let a_elt = match self.set(a, b_elt) {
            Ok(old) => old,
            Err(_) => unreachable!("set at a lesser index than swap"),
        };
        match self.insert(b, a_elt) {
            Ok(()) => {}
            Err(_) => {
                unreachable!("insert at a previously removed place")
            }
        };
        true
    }

    /// Get the `element` at `index` in the list.
    ///
    /// ```
    /// # use btreelist::btreelist;
    /// let list = btreelist![10, 40, 30];
    /// assert_eq!(list.get(1), Some(&40));
    /// assert_eq!(list.get(3), None);
    /// ```
    pub fn get(&self, index: usize) -> Option<&T> {
        self.root_node.as_ref().and_then(|n| n.get(index))
    }

    /// Get the and `element` at `index` in the list.
    ///
    /// ```
    /// # use btreelist::btreelist;
    /// let list = &mut btreelist![0, 1, 2];
    /// if let Some(elem) = list.get_mut(1) {
    ///     *elem = 42;
    /// }
    /// assert_eq!(*list, btreelist![0, 42, 2]);
    /// ```
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.root_node.as_mut().and_then(|n| n.get_mut(index))
    }

    /// Get the first element in the list if it exists.
    ///
    /// ```
    /// # use btreelist::btreelist;
    /// let list = btreelist![10, 40, 30];
    /// assert_eq!(list.first(), Some(&10));
    /// ```
    pub fn first(&self) -> Option<&T> {
        self.get(0)
    }

    /// Get the first element in the list if it exists.
    ///
    /// ```
    /// # use btreelist::btreelist;
    /// let list = &mut btreelist![0, 1, 2];
    /// if let Some(elem) = list.first_mut() {
    ///     *elem = 42;
    /// }
    /// assert_eq!(*list, btreelist![42, 1, 2]);
    /// ```
    pub fn first_mut(&mut self) -> Option<&mut T> {
        self.get_mut(0)
    }

    /// Get the last element in the list if it exists.
    ///
    /// ```
    /// # use btreelist::btreelist;
    /// let list = btreelist![10, 40, 30];
    /// assert_eq!(list.last(), Some(&30));
    /// ```
    pub fn last(&self) -> Option<&T> {
        self.get(self.len() - 1)
    }

    /// Get the last element in the list if it exists.
    ///
    /// ```
    /// # use btreelist::btreelist;
    /// let list = &mut btreelist![0, 1, 2];
    /// if let Some(elem) = list.last_mut() {
    ///     *elem = 42;
    /// }
    /// assert_eq!(*list, btreelist![0, 1, 42]);
    /// ```
    pub fn last_mut(&mut self) -> Option<&mut T> {
        self.get_mut(self.len() - 1)
    }
}

impl<T, const B: usize> BTreeListNode<T, B> {
    fn new() -> Self {
        Self {
            elements: Vec::new(),
            children: Vec::new(),
            length: 0,
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.length
    }

    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    fn is_full(&self) -> bool {
        let max = 2 * B - 1;
        assert!(
            self.elements.len() <= max,
            "node shouldn't be over full-size"
        );
        self.elements.len() == max
    }

    /// Returns the child index and the given index adjusted for the cumulative index before that
    /// child.
    fn find_child_index(&self, index: usize) -> Option<(usize, usize)> {
        let mut cumulative_len = 0;
        for (child_index, child) in self.children.iter().enumerate() {
            if cumulative_len + child.len() >= index {
                return Some((child_index, index - cumulative_len));
            } else {
                cumulative_len += child.len() + 1;
            }
        }
        None
    }

    fn insert_into_non_full_node(&mut self, index: usize, element: T) -> Result<(), T> {
        assert!(!self.is_full());
        if self.is_leaf() {
            self.length += 1;
            if index <= self.elements.len() {
                self.elements.insert(index, element);
                Ok(())
            } else {
                Err(element)
            }
        } else if let Some((child_index, sub_index)) = self.find_child_index(index) {
            let child = &mut self.children[child_index];

            if child.is_full() {
                self.split_child(child_index);

                // child structure has changed so we need to find the index again
                if let Some((child_index, sub_index)) = self.find_child_index(index) {
                    let child = &mut self.children[child_index];
                    child.insert_into_non_full_node(sub_index, element)?;
                } else {
                    return Err(element);
                }
            } else {
                child.insert_into_non_full_node(sub_index, element)?;
            }
            self.length += 1;
            Ok(())
        } else {
            Err(element)
        }
    }

    // A utility function to split the child `full_child_index` of this node
    // Note that `full_child_index` must be full when this function is called.
    fn split_child(&mut self, full_child_index: usize) {
        let original_len_self = self.len();

        // Create a new node which is going to store (B-1) keys
        // of the full child.
        let mut successor_sibling = BTreeListNode::new();

        let full_child = &mut self.children[full_child_index];
        let original_len = full_child.len();
        assert!(full_child.is_full());

        successor_sibling.elements = full_child.elements.split_off(B);

        if !full_child.is_leaf() {
            successor_sibling.children = full_child.children.split_off(B);
        }

        let middle = full_child.elements.pop().unwrap();

        full_child.length =
            full_child.elements.len() + full_child.children.iter().map(|c| c.len()).sum::<usize>();

        successor_sibling.length = successor_sibling.elements.len()
            + successor_sibling
                .children
                .iter()
                .map(|c| c.len())
                .sum::<usize>();

        let z_len = successor_sibling.len();

        let full_child_len = full_child.len();

        self.children
            .insert(full_child_index + 1, successor_sibling);

        self.elements.insert(full_child_index, middle);

        assert_eq!(full_child_len + z_len + 1, original_len);

        assert_eq!(original_len_self, self.len());
    }

    fn remove_from_leaf(&mut self, index: usize) -> Option<T> {
        self.length -= 1;
        if index < self.elements.len() {
            Some(self.elements.remove(index))
        } else {
            None
        }
    }

    fn remove_element_from_non_leaf(&mut self, index: usize, element_index: usize) -> Option<T> {
        self.length -= 1;
        if self.children[element_index].elements.len() >= B {
            let total_index = self.cumulative_index(element_index);
            // recursively delete index - 1 in predecessor_node
            let predecessor = self.children[element_index].remove(index - 1 - total_index)?;
            // replace element with that one
            Some(mem::replace(&mut self.elements[element_index], predecessor))
        } else if self.children[element_index + 1].elements.len() >= B {
            // recursively delete index + 1 in successor_node
            let total_index = self.cumulative_index(element_index + 1);
            let successor = self.children[element_index + 1].remove(index + 1 - total_index)?;
            // replace element with that one
            Some(mem::replace(&mut self.elements[element_index], successor))
        } else {
            let middle_element = self.elements.remove(element_index);
            let successor_child = self.children.remove(element_index + 1);
            self.children[element_index].merge(middle_element, successor_child);

            let total_index = self.cumulative_index(element_index);
            self.children[element_index].remove(index - total_index)
        }
    }

    fn cumulative_index(&self, child_index: usize) -> usize {
        self.children[0..child_index]
            .iter()
            .map(|c| c.len() + 1)
            .sum()
    }

    fn remove_from_internal_child(&mut self, index: usize, mut child_index: usize) -> Option<T> {
        if self.children[child_index].elements.len() < B
            && if child_index > 0 {
                self.children[child_index - 1].elements.len() < B
            } else {
                true
            }
            && if child_index + 1 < self.children.len() {
                self.children[child_index + 1].elements.len() < B
            } else {
                true
            }
        {
            // if the child and its immediate siblings have B-1 elements merge the child
            // with one sibling, moving an element from this node into the new merged node
            // to be the median

            if child_index > 0 {
                let middle = self.elements.remove(child_index - 1);

                // use the predessor sibling
                let successor = self.children.remove(child_index);
                child_index -= 1;

                self.children[child_index].merge(middle, successor);
            } else {
                let middle = self.elements.remove(child_index);

                // use the sucessor sibling
                let successor = self.children.remove(child_index + 1);

                self.children[child_index].merge(middle, successor);
            }
        } else if self.children[child_index].elements.len() < B {
            if child_index > 0
                && self
                    .children
                    .get(child_index - 1)
                    .map_or(false, |c| c.elements.len() >= B)
            {
                let last_element = self.children[child_index - 1].elements.pop().unwrap();
                assert!(!self.children[child_index - 1].elements.is_empty());
                self.children[child_index - 1].length -= 1;

                let parent_element =
                    mem::replace(&mut self.elements[child_index - 1], last_element);

                self.children[child_index]
                    .elements
                    .insert(0, parent_element);
                self.children[child_index].length += 1;

                if let Some(last_child) = self.children[child_index - 1].children.pop() {
                    self.children[child_index - 1].length -= last_child.len();
                    self.children[child_index].length += last_child.len();
                    self.children[child_index].children.insert(0, last_child);
                }
            } else if self
                .children
                .get(child_index + 1)
                .map_or(false, |c| c.elements.len() >= B)
            {
                let first_element = self.children[child_index + 1].elements.remove(0);
                self.children[child_index + 1].length -= 1;

                assert!(!self.children[child_index + 1].elements.is_empty());

                let parent_element = mem::replace(&mut self.elements[child_index], first_element);

                self.children[child_index].length += 1;
                self.children[child_index].elements.push(parent_element);

                if !self.children[child_index + 1].is_leaf() {
                    let first_child = self.children[child_index + 1].children.remove(0);
                    self.children[child_index + 1].length -= first_child.len();
                    self.children[child_index].length += first_child.len();

                    self.children[child_index].children.push(first_child);
                }
            }
        }
        self.length -= 1;
        let total_index = self.cumulative_index(child_index);
        self.children[child_index].remove(index - total_index)
    }

    fn check(&self) -> usize {
        let l = self.elements.len() + self.children.iter().map(|c| c.check()).sum::<usize>();
        assert_eq!(self.len(), l);

        l
    }

    pub(crate) fn remove(&mut self, index: usize) -> Option<T> {
        let original_len = self.len();
        if self.is_leaf() {
            let v = self.remove_from_leaf(index);
            assert_eq!(original_len, self.len() + 1);
            debug_assert_eq!(self.check(), self.len());
            v
        } else {
            let mut total_index = 0;
            for (child_index, child) in self.children.iter().enumerate() {
                match (total_index + child.len()).cmp(&index) {
                    Ordering::Less => {
                        // should be later on in the loop
                        total_index += child.len() + 1;
                        continue;
                    }
                    Ordering::Equal => {
                        let v = self.remove_element_from_non_leaf(
                            index,
                            min(child_index, self.elements.len() - 1),
                        );
                        assert_eq!(original_len, self.len() + 1);
                        debug_assert_eq!(self.check(), self.len());
                        return v;
                    }
                    Ordering::Greater => {
                        let v = self.remove_from_internal_child(index, child_index);
                        assert_eq!(original_len, self.len() + 1);
                        debug_assert_eq!(self.check(), self.len());
                        return v;
                    }
                }
            }
            None
        }
    }

    fn merge(&mut self, middle: T, successor_sibling: BTreeListNode<T, B>) {
        self.elements.push(middle);
        self.elements.extend(successor_sibling.elements);
        self.children.extend(successor_sibling.children);
        self.length += successor_sibling.length + 1;
        assert!(self.is_full());
    }

    pub(crate) fn set(&mut self, index: usize, element: T) -> Result<T, T> {
        if self.is_leaf() {
            let old_element = self.elements.get_mut(index).unwrap();
            Ok(mem::replace(old_element, element))
        } else {
            let mut cumulative_len = 0;
            for (child_index, child) in self.children.iter_mut().enumerate() {
                match (cumulative_len + child.len()).cmp(&index) {
                    Ordering::Less => {
                        cumulative_len += child.len() + 1;
                    }
                    Ordering::Equal => {
                        let old_element = self.elements.get_mut(child_index).unwrap();
                        return Ok(mem::replace(old_element, element));
                    }
                    Ordering::Greater => {
                        return child.set(index - cumulative_len, element);
                    }
                }
            }
            // can't set so return the original element
            Err(element)
        }
    }

    pub(crate) fn get(&self, index: usize) -> Option<&T> {
        if self.is_leaf() {
            return self.elements.get(index);
        } else {
            let mut cumulative_len = 0;
            for (child_index, child) in self.children.iter().enumerate() {
                match (cumulative_len + child.len()).cmp(&index) {
                    Ordering::Less => {
                        cumulative_len += child.len() + 1;
                    }
                    Ordering::Equal => {
                        return self.elements.get(child_index);
                    }
                    Ordering::Greater => {
                        return child.get(index - cumulative_len);
                    }
                }
            }
        }
        None
    }

    pub(crate) fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if self.is_leaf() {
            return self.elements.get_mut(index);
        } else {
            let mut cumulative_len = 0;
            for (child_index, child) in self.children.iter_mut().enumerate() {
                match (cumulative_len + child.len()).cmp(&index) {
                    Ordering::Less => {
                        cumulative_len += child.len() + 1;
                    }
                    Ordering::Equal => {
                        return self.elements.get_mut(child_index);
                    }
                    Ordering::Greater => {
                        return child.get_mut(index - cumulative_len);
                    }
                }
            }
        }
        None
    }
}

impl<T> Default for BTreeList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> PartialEq for BTreeList<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().zip(other.iter()).all(|(a, b)| a == b)
    }
}

impl<'a, T, const B: usize> IntoIterator for &'a BTreeList<T, B> {
    type Item = &'a T;

    type IntoIter = Iter<'a, T, B>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            inner: self,
            index: 0,
            index_back: self.len(),
        }
    }
}

impl<T, const B: usize> IntoIterator for BTreeList<T, B> {
    type Item = T;

    type IntoIter = OwnedIter<T, B>;

    fn into_iter(self) -> Self::IntoIter {
        OwnedIter { inner: self }
    }
}

impl<T> Extend<T> for BTreeList<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push_back(item)
        }
    }
}

impl<T> FromIterator<T> for BTreeList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut l = BTreeList::new();
        for item in iter {
            l.push_back(item);
        }
        l
    }
}

impl<T> Index<usize> for BTreeList<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<T> IndexMut<usize> for BTreeList<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

#[cfg(test)]
mod tests {

    use crate::btreelist;

    use super::*;

    #[test]
    fn push_back() {
        let mut t = BTreeList::default();

        t.push_back(());
        t.push_back(());
        t.push_back(());
        t.push_back(());
        t.push_back(());
        t.push_back(());
        t.push_back(());
        t.push_back(());
    }

    #[test]
    fn insert() {
        let mut t = BTreeList::default();

        t.insert(0, ()).unwrap();
        t.insert(1, ()).unwrap();
        t.insert(0, ()).unwrap();
        t.insert(0, ()).unwrap();
        t.insert(0, ()).unwrap();
        t.insert(3, ()).unwrap();
        t.insert(4, ()).unwrap();
    }

    #[test]
    fn insert_book() {
        let mut t = BTreeList::default();

        for i in 0..100 {
            t.insert(i % 2, ()).unwrap();
        }
    }

    #[test]
    fn insert_book_vec() {
        let mut t = BTreeList::default();
        let mut v = Vec::new();

        for i in 0..100 {
            t.insert(i % 3, ()).unwrap();
            v.insert(i % 3, ());

            assert_eq!(v, t.iter().copied().collect::<Vec<_>>())
        }
    }

    #[test]
    fn iter_forth_back() {
        let mut t = BTreeList::default();

        t.push_back(1);
        t.push_back(2);
        t.push_back(3);
        t.push_back(4);
        t.push_back(5);

        let mut i = t.iter();
        assert_eq!(i.next(), Some(&1));
        assert_eq!(i.next(), Some(&2));
        assert_eq!(i.next(), Some(&3));
        assert_eq!(i.next(), Some(&4));
        assert_eq!(i.next(), Some(&5));
        assert_eq!(i.next(), None);

        let mut i = t.iter();
        assert_eq!(i.next_back(), Some(&5));
        assert_eq!(i.next_back(), Some(&4));
        assert_eq!(i.next_back(), Some(&3));
        assert_eq!(i.next_back(), Some(&2));
        assert_eq!(i.next_back(), Some(&1));
        assert_eq!(i.next_back(), None);

        let mut i = t.iter();
        assert_eq!(i.next(), Some(&1));
        assert_eq!(i.next_back(), Some(&5));
        assert_eq!(i.next_back(), Some(&4));
        assert_eq!(i.next_back(), Some(&3));
        assert_eq!(i.next_back(), Some(&2));
        assert_eq!(i.next_back(), None);

        let mut i = t.iter();
        assert_eq!(i.next(), Some(&1));
        assert_eq!(i.next(), Some(&2));
        assert_eq!(i.next(), Some(&3));
        assert_eq!(i.next(), Some(&4));
        assert_eq!(i.next_back(), Some(&5));
        assert_eq!(i.next_back(), None);
    }

    #[test]
    fn first_last() {
        let mut t = BTreeList::default();
        t.push_back(1);
        t.push_back(2);
        t.push_back(3);

        assert_eq!(t.first(), Some(&1));
        assert_eq!(t.first_mut(), Some(&mut 1));
        assert_eq!(t.last(), Some(&3));
        assert_eq!(t.last_mut(), Some(&mut 3));
    }

    #[test]
    fn pop() {
        let mut t = BTreeList::default();

        t.push_back(1);
        t.push_back(2);
        t.push_back(3);

        assert_eq!(t.pop_back(), Some(3));
        assert_eq!(t.pop_back(), Some(2));
        assert_eq!(t.pop_back(), Some(1));
        assert_eq!(t.pop_back(), None);
    }

    #[test]
    fn set_no_panic() {
        let mut t = BTreeList::default();
        assert_eq!(t.set(0, 1), Err(1));
        t.push(1);
        assert_eq!(t.set(0, 2), Ok(1));
    }

    #[test]
    fn remove_no_panic() {
        let mut t = BTreeList::default();
        assert_eq!(t.remove(0), None);
        assert_eq!(t.remove(1), None);
        t.push(1);
        assert_eq!(t.remove(0), Some(1));
    }

    #[test]
    fn insert_no_panic() {
        let mut t = BTreeList::default();
        assert_eq!(t.insert(10, 1), Err(1));
        assert_eq!(t.insert(1, 1), Err(1));
        assert_eq!(t.insert(0, 1), Ok(()));
        assert_eq!(t.insert(10, 1), Err(1));
    }

    #[test]
    fn generic_b() {
        fn push_a_few<const B: usize>(mut t: BTreeList<usize, B>) {
            t.push(1);
            t.push(2);
            t.push(3);
        }

        let t: BTreeList<usize> = BTreeList::new();
        let t6: BTreeList<usize, 6> = BTreeList::new();
        let t3: BTreeList<usize, 3> = BTreeList::new();
        let t7: BTreeList<usize, 7> = BTreeList::new();
        let t32: BTreeList<usize, 32> = BTreeList::new();

        push_a_few(t);
        push_a_few(t6);
        push_a_few(t3);
        push_a_few(t7);
        push_a_few(t32);
    }

    #[test]
    fn swap() {
        let mut t = BTreeList::default();

        t.push(1);
        t.push(2);
        t.push(3);

        t.swap(0, 2);
        assert_eq!(t, btreelist![3, 2, 1]);
        t.swap(2, 0);
        assert_eq!(t, btreelist![1, 2, 3]);
        t.swap(1, 0);
        assert_eq!(t, btreelist![2, 1, 3]);
        t.swap(0, 1);
        assert_eq!(t, btreelist![1, 2, 3]);
        assert!(!t.swap(0, 4));
        assert!(!t.swap(5, 4));
    }

    #[cfg(release)]
    fn arb_indices() -> impl Strategy<Value = Vec<usize>> {
        proptest::collection::vec(any::<usize>(), 0..1000).prop_map(|v| {
            let mut len = 0;
            v.into_iter()
                .map(|i| {
                    len += 1;
                    i % len
                })
                .collect::<Vec<_>>()
        })
    }

    use proptest::prelude::*;

    proptest! {

        #[test]
        #[cfg(release)]
        fn proptest_insert(indices in arb_indices()) {
            let mut t = BTreeList::<usize, 3>::new();
            let mut v = Vec::new();

            for i in indices{
                if i <= v.len() {
                    t.insert(i % 3,  i);
                    v.insert(i % 3, i);
                } else {
                    return Err(proptest::test_runner::TestCaseError::reject("index out of bounds"))
                }

                assert_eq!(v, t.iter().copied().collect::<Vec<_>>())
            }
        }

    }

    proptest! {

        #[test]
        #[cfg(release)]
        fn proptest_remove(inserts in arb_indices(), removes in arb_indices()) {
            let mut t = BTreeList::<usize, 3>::new();
            let mut v = Vec::new();

            for i in inserts {
                if i <= v.len() {
                    t.insert(i,  i);
                    v.insert(i, i);
                } else {
                    return Err(proptest::test_runner::TestCaseError::reject("index out of bounds"))
                }

                assert_eq!(v, t.iter().copied().collect::<Vec<_>>())
            }

            for i in removes {
                if i < v.len() {
                    let tr = t.remove(i);
                    let vr = v.remove(i);
                    assert_eq!(tr, vr);
                } else {
                    return Err(proptest::test_runner::TestCaseError::reject("index out of bounds"))
                }

                assert_eq!(v, t.iter().copied().collect::<Vec<_>>())
            }
        }

    }
}
