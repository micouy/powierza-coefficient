use criterion::{black_box, criterion_group, criterion_main, Criterion};

use powierza_coefficient::powierża_coefficient;

use strsim::levenshtein;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Levenshtein distance", |b| {
        b.iter(|| {
            levenshtein(
                black_box("abc_jkl_mno_xyz"),
                black_box(
                    "xyz_mno_jkl_abc_mno_jkl_xyz_abc_mon_jkl_mno_xyz_xyz",
                ),
            )
        })
    });

    c.bench_function("Powierża coefficient", |b| {
        b.iter(|| {
            powierża_coefficient(
                black_box("abc_jkl_mno_xyz"),
                black_box(
                    "xyz_mno_jkl_abc_mno_jkl_xyz_abc_mon_jkl_mno_xyz_xyz",
                ),
            )
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
