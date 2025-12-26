//! Benchmark entry point for JSONLT.
//!
//! Run benchmarks with: `cargo bench`

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use jsonlt::{Record, Table};

fn bench_table_insert(c: &mut Criterion) {
    c.bench_function("table_insert_100", |b| {
        b.iter(|| {
            let mut table = Table::new();
            for i in 0..100 {
                table.insert(format!("key-{i}"), Record::new());
            }
            black_box(table)
        });
    });
}

fn bench_table_lookup(c: &mut Criterion) {
    let mut table = Table::new();
    for i in 0..1000 {
        table.insert(format!("key-{i}"), Record::new());
    }

    c.bench_function("table_lookup_1000", |b| {
        b.iter(|| {
            for i in 0..1000 {
                black_box(table.get(&format!("key-{i}")));
            }
        });
    });
}

fn bench_table_iteration(c: &mut Criterion) {
    let mut table = Table::new();
    for i in 0..1000 {
        table.insert(format!("key-{i}"), Record::new());
    }

    c.bench_function("table_iteration_1000", |b| {
        b.iter(|| {
            for (key, record) in table.iter() {
                black_box((key, record));
            }
        });
    });
}

#[cfg(feature = "serde")]
fn bench_record_serialization(c: &mut Criterion) {
    use serde_json::json;

    let value = json!({
        "name": "Test Record",
        "count": 42,
        "tags": ["a", "b", "c"],
        "nested": {
            "field1": "value1",
            "field2": 123
        }
    });

    c.bench_function("record_from_value", |b| {
        b.iter(|| black_box(Record::from_value(value.clone())));
    });
}

#[cfg(not(feature = "serde"))]
fn bench_record_serialization(_c: &mut Criterion) {
    // No-op when serde is not enabled
}

criterion_group!(
    benches,
    bench_table_insert,
    bench_table_lookup,
    bench_table_iteration,
    bench_record_serialization,
);

criterion_main!(benches);
