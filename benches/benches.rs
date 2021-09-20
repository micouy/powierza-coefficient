use criterion::{black_box, criterion_group, criterion_main, Criterion};
use powierza_distance::powierza_distance;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("bench 1", |b| b.iter(|| {
		powierza_distance(
			black_box("abc_jkl_mno_xyz"),
			black_box("xyz_mno_jkl_abc_mno_jkl_xyz_abc_mon_jkl_mno_xyz_xyz")
		)
	}));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
