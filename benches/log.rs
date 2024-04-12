use criterion::{criterion_group, criterion_main, Criterion};
use logflume::{info, Level};
use std::fs;
use std::path::Path;

fn bench_function() {
    info!("test");
}

fn criterion_benchmark(c: &mut Criterion) {
    if Path::new("test.log").exists() {
        fs::remove_file("test.log").expect("Cannot delete benchmark log file.");
    }
    logflume::Logger::new()
        .level(Level::Debug)
        .cpu(7)
        .file("bench.log")
        .init()
        .expect("Unable to construct logger");

    c.bench_function("log test", |b| b.iter(|| bench_function()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
