/// Create a [`BTreeList`](crate::BTreeList).
///
/// ```
/// # use btreelist::btreelist;
/// # fn main() {
/// btreelist![1];
/// btreelist![1, 2, 3];
/// btreelist![1; 3];
/// # }
/// ```
#[macro_export]
macro_rules! btreelist {
    () => {
        $crate::BTreeList::default()
    };
    ($elem:expr; $n:expr) => {
        {
            let mut t = $crate::BTreeList::default();
            for _ in 0..$n {
                t.push($elem)
            }
            t
        }
    };
    ($($x:expr),+ $(,)?) => {
        {
            let mut t = $crate::BTreeList::default();
            $(t.push($x);)+
            t
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::BTreeList;

    #[test]
    fn mc() {
        let _: BTreeList<()> = btreelist![];
        btreelist![1];
        btreelist![1, 2, 3];
        btreelist![1; 3];
    }
}
