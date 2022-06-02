#![no_main]
use btreelist::BTreeList;
use libfuzzer_sys::fuzz_target;

pub enum Action {
    Insert(usize),
    Remove(usize),
}

fuzz_target!(|data: Vec<Action>| {
    let mut sq = BTreeList::new();
    let mut v = Vec::new();

    let mut val = 0;
    for action in data {
        val += 1;
        match action {
            Action::Insert(index) => {
                sq.insert(index, val);
                v.insert(index, val);
            }
            Action::Remove(index) => {
                sq.remove(index);
                v.remove(index);
            }
        }
    }

    assert_eq!(sq, v)
});
