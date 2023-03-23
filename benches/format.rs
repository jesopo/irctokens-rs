use criterion::{criterion_group, criterion_main, Criterion};
use irctokens::Line;
use std::collections::BTreeMap;

fn criterion_benchmark(c: &mut Criterion) {
    let line = Line {
        tags: Some(BTreeMap::from([
            ("tag1".to_string(), Some("tag1value".to_string())),
            ("tag2".to_string(), None),
            ("tag3".to_string(), Some("a;a".to_string())),
        ])),
        source: Some(b"source".to_vec()),
        command: "COMMAND".to_string(),
        args: Vec::from([
            b"arg1".to_vec(),
            b"arg2".to_vec(),
            b"arg3 with space".to_vec(),
        ]),
    };
    c.bench_function("basic", |b| b.iter(|| line.format()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
