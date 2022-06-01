use btree_vec::BTreeVec;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use sequence_tree::SequenceTree;

macro_rules! push {
    ($name:ident, $v:ident) => {
        paste::item! {
            fn [< push_ $name >] (n: u64) {
                let mut v = $v::new();
                for i in 0..n {
                    v.push(i);
                }
            }
        }
    };
}

macro_rules! insert {
    ($name:ident, $v:ident) => {
        paste::item! {
            fn [< insert_ $name >] (n: u64) {
                let mut v = $v::new();
                for i in 0..n {
                    v.insert(0, i);
                }
            }
        }
    };
}

macro_rules! remove {
    ($name:ident, $v:ident) => {
        paste::item! {
            fn [< remove_ $name >] (n: u64) {
                let mut v = $v::new();
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

macro_rules! iter {
    ($name:ident, $v:ident) => {
        paste::item! {
            fn [< iter_ $name >] (n: u64) {
                let mut v = $v::new();
                for i in 0..n {
                    v.push(i);
                }
                v.iter().count();
            }
        }
    };
}

macro_rules! impls {
    (($name:ident, $v:ident)) => {
        push!($name, $v);
        insert!($name, $v);
        remove!($name, $v);
        iter!($name, $v);
    };
    (($name:ident, $v:ident), $($others:tt),+) => {
        impls!(($name, $v));
        impls!($($others),+);
    };
}

impls![(vec, Vec), (sqt, SequenceTree), (btv, BTreeVec)];

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
                    group.bench_with_input(BenchmarkId::new("sqt", size), &size, |b, &size| {
                        b.iter(|| [< $name _sqt >] (size))
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

    bg!("push", "insert", "remove", "iter");
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
