use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::fs;
// use rayon::prelude::*;

use sudoku_rs::{Sudoku, solve_sudoku};

fn bench_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("sudoku");

    let file = fs::read_to_string("./benches/0_Easy_000.adk").expect("Couldn't find the game file");
    // let length = line.split("\n").collect::<Vec<_>>().len();
    let length = 10;

    group.throughput(Throughput::Elements(length as u64));
    group.sample_size(10);
    group.bench_with_input(BenchmarkId::new("solve", length), &file, |benches, input| {
        benches.iter(|| {
            let line = input.split('\n').take(10).collect::<Vec<_>>();
            line.iter().map(|s| {
                let mut g = Sudoku::new(s);
                solve_sudoku(&mut g);
            }).for_each(drop);
        })
    });

    group.finish();
}

criterion_group!(benches, bench_throughput);
criterion_main!(benches);
