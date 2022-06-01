use btree_vec::BTreeVec;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use sequence_tree::SequenceTree;

fn push_vec(n: u64) {
    let mut v = Vec::new();
    for i in 0..n {
        v.push(i);
    }
}

fn push_st(n: u64) {
    let mut v = SequenceTree::new();
    for i in 0..n {
        v.push(i);
    }
}

fn push_btreevec(n: u64) {
    let mut v = BTreeVec::new();
    for i in 0..n {
        v.push(i);
    }
}

fn insert_vec(n: u64) {
    let mut v = Vec::new();
    for i in 0..n {
        v.insert(0, i);
    }
}

fn insert_st(n: u64) {
    let mut v = SequenceTree::new();
    for i in 0..n {
        v.insert(0, i);
    }
}

fn insert_btreevec(n: u64) {
    let mut v = BTreeVec::new();
    for i in 0..n {
        v.insert(0, i);
    }
}

fn remove_vec(n: u64) {
    let mut v = Vec::new();
    for i in 0..n {
        v.push(i);
    }
    for _ in 0..n {
        v.remove(0);
    }
}

fn remove_st(n: u64) {
    let mut v = SequenceTree::new();
    for i in 0..n {
        v.push(i);
    }
    for _ in 0..n {
        v.remove(0);
    }
}

fn remove_btreevec(n: u64) {
    let mut v = BTreeVec::new();
    for i in 0..n {
        v.push(i);
    }
    for _ in 0..n {
        v.remove(0);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("push");
    for size in [100, 1000, 10000] {
        group.throughput(criterion::Throughput::Elements(size));
        group.bench_with_input(BenchmarkId::new("vec", size), &size, |b, &size| {
            b.iter(|| push_vec(size))
        });
        group.bench_with_input(BenchmarkId::new("sqt", size), &size, |b, &size| {
            b.iter(|| push_st(size))
        });
        group.bench_with_input(BenchmarkId::new("btv", size), &size, |b, &size| {
            b.iter(|| push_btreevec(size))
        });
    }
    group.finish();

    c.bench_function("insert vec 100", |b| b.iter(|| insert_vec(black_box(100))));
    c.bench_function("insert st  100", |b| b.iter(|| insert_st(black_box(100))));
    c.bench_function("insert btv 100", |b| {
        b.iter(|| insert_btreevec(black_box(100)))
    });

    c.bench_function("remove vec 100", |b| b.iter(|| remove_vec(black_box(100))));
    c.bench_function("remove st  100", |b| b.iter(|| remove_st(black_box(100))));
    c.bench_function("remove btv 100", |b| {
        b.iter(|| remove_btreevec(black_box(100)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
