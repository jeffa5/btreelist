use btree_vec::BTreeVec;
use btreelist::BTreeList;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

macro_rules! empty {
    (vec) => {
        Vec::new()
    };
    (btv) => {
        BTreeVec::new()
    };
    (btl) => {
        BTreeList::default()
    };
}

macro_rules! push {
    ($name:ident) => {
        paste::item! {
            fn [< push_ $name >] (n: u64) {
                let mut v = empty!($name);
                for i in 0..n {
                    v.push(i);
                }
            }
        }
    };
}

macro_rules! pop {
    ($name:ident) => {
        paste::item! {
            fn [< pop_ $name >] (n: u64) {
                let mut v = empty!($name);
                for i in 0..n {
                    v.push(i);
                }
                for _ in 0..n {
                    v.pop();
                }
            }
        }
    };
}

macro_rules! insert {
    ($name:ident) => {
        paste::item! {
            fn [< insert_ $name >] (n: u64) {
                let mut v = empty!($name);
                for i in 0..n {
                    let _ = v.insert(0, i);
                }
            }
        }
    };
}

macro_rules! remove {
    ($name:ident) => {
        paste::item! {
            fn [< remove_ $name >] (n: u64) {
                let mut v = empty!($name);
                for i in 0..n {
                    v.push(i);
                }
                for _ in 0..n {
                    v.remove(0);
                }
            }
        }
    };
}

macro_rules! get {
    ($name:ident) => {
        paste::item! {
            fn [< get_ $name >] (n: u64) {
                let mut v = empty!($name);
                for i in 0..n {
                    v.push(i);
                }
                for i in 0..n {
                    let _ = v.get(i as usize);
                }
            }
        }
    };
}

macro_rules! iter {
    ($name:ident) => {
        paste::item! {
            fn [< iter_ $name >] (n: u64) {
                let mut v = empty!($name);
                for i in 0..n {
                    v.push(i);
                }
                v.iter().count();
            }
        }
    };
}

macro_rules! impls {
    ($name:ident) => {
        push!($name);
        pop!($name);
        insert!($name);
        remove!($name);
        get!($name);
        iter!($name);
    };
    ($name:ident, $($others:tt),+) => {
        impls!($name);
        impls!($($others),+);
    };
}

impls![vec, btl, btv];

fn criterion_benchmark(c: &mut Criterion) {
    macro_rules! bg {
        ($name:expr) => {
            paste::item! {
                let mut group = c.benchmark_group($name);
                for size in [100, 1000, 10000] {
                    group.throughput(criterion::Throughput::Elements(size));
                    group.bench_with_input(BenchmarkId::new("vec", size), &size, |b, &size| {
                        b.iter(|| [< $name _vec >] (size))
                    });
                    group.bench_with_input(BenchmarkId::new("btl", size), &size, |b, &size| {
                        b.iter(|| [< $name _btl >] (size))
                    });
                    group.bench_with_input(BenchmarkId::new("btv", size), &size, |b, &size| {
                        b.iter(|| [< $name _btv >] (size))
                    });
                }
                group.finish();
            };
        };
        ($name:expr, $($names:expr),+) => {
            bg!($name);
            bg!($($names),+)
        }
    }

    bg!["push", "pop", "insert", "remove", "get", "iter"];
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
