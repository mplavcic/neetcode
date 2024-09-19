use criterion::{black_box, criterion_group, criterion_main, Criterion};
use neetcode::arrays_and_hashing::is_anagram::is_anagram;
pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("is_anagram", |b| b.iter(|| is_anagram("mateo", "oetam")));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
