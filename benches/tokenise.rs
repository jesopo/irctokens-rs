use criterion::{criterion_group, criterion_main, Criterion};
use irctokens::Line;

fn basic() {
    Line::tokenise(b"@tag1=tag1value;tag2=;tag3 :source COMMAND arg1 arg2 :arg3 with space").unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("basic", |b| b.iter(basic));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
