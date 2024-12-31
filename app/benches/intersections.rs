use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sg_geometry::{intersections::segment_segment, LineSegment};

const SECONDS_TO_BENCHMARK: u64 = 10;

pub fn segment_segment_individual_types(c: &mut Criterion) {
    let mut group = c.benchmark_group("segment-segment individual types");
    group.sample_size(1000);
    group.measurement_time(std::time::Duration::from_secs(SECONDS_TO_BENCHMARK));

    let s1 = LineSegment::from([[-2.0, -2.0], [4.0, 4.0]]);
    let s2 = LineSegment::from([[6.0, 6.0], [10.0, 10.0]]);
    group.bench_function("segment-segment bounding box miss", |b| b.iter(|| segment_segment(black_box(s1), black_box(s2))));

    let s1 = LineSegment::from([[0.0, 0.0], [10.0, 10.0]]);
    let s2 = LineSegment::from([[2.0, 2.0], [6.0, 6.0]]);
    group.bench_function("segment-segment collinear (hit)", |b| b.iter(|| segment_segment(black_box(s1), black_box(s2))));

    let s1 = LineSegment::from([[4.0, 4.0], [12.0, 12.0]]);
    let s2 = LineSegment::from([[6.0, 8.0], [8.0, 10.0]]);
    group.bench_function("segment-segment parallel (miss)", |b| b.iter(|| segment_segment(black_box(s1), black_box(s2))));

    let s1 = LineSegment::from([[0.0, 0.0], [10.0, 10.0]]);
    let s2 = LineSegment::from([[2.0, 2.0], [17.0, 4.0]]);
    group.bench_function("segment-segment general hit", |b| b.iter(|| segment_segment(black_box(s1), black_box(s2))));

    let s1 = LineSegment::from([[0.0, 0.0], [2.0, 2.0]]);
    let s2 = LineSegment::from([[1.0, 4.0], [4.0, 0.0]]);
    group.bench_function("segment-segment general miss", |b| b.iter(|| segment_segment(black_box(s1), black_box(s2))));

    group.finish();
}

criterion_group!(benches, segment_segment_individual_types);
criterion_main!(benches);
