use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use pi_stream_terminal::{PiAlgorithm, compute_pi};

fn benchmark_algorithms_small(c: &mut Criterion) {
    let mut group = c.benchmark_group("Pi Algorithms - Small (50 digits)");

    let algorithms = [
        (PiAlgorithm::ChudnovskyNaive, "Chudnovsky_Naive"),
        (PiAlgorithm::ChudnovskyBinarySplitting, "Chudnovsky_BinarySplitting"),
        (PiAlgorithm::MachinFormula, "Machin_Formula"),
        (PiAlgorithm::BorweinQuadratic, "Borwein_Quadratic"),
    ];

    for (algorithm, name) in algorithms {
        group.bench_with_input(BenchmarkId::new(name, 50), &50, |b, &digits| {
            b.iter(|| compute_pi(black_box(digits), black_box(algorithm)))
        });
    }

    group.finish();
}

fn benchmark_algorithms_medium(c: &mut Criterion) {
    let mut group = c.benchmark_group("Pi Algorithms - Medium (500 digits)");

    let algorithms = [
        (PiAlgorithm::ChudnovskyNaive, "Chudnovsky_Naive"),
        (PiAlgorithm::ChudnovskyBinarySplitting, "Chudnovsky_BinarySplitting"),
        (PiAlgorithm::MachinFormula, "Machin_Formula"),
        (PiAlgorithm::BorweinQuadratic, "Borwein_Quadratic"),
    ];

    for (algorithm, name) in algorithms {
        group.bench_with_input(BenchmarkId::new(name, 500), &500, |b, &digits| {
            b.iter(|| compute_pi(black_box(digits), black_box(algorithm)))
        });
    }

    group.finish();
}

fn benchmark_binary_splitting_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("Binary Splitting - Scaling");

    let digit_counts = [100, 500, 1000, 2000, 5000];

    for &digits in &digit_counts {
        group.bench_with_input(
            BenchmarkId::new("BinarySplitting", digits),
            &digits,
            |b, &digits| {
                b.iter(|| compute_pi(black_box(digits), black_box(PiAlgorithm::ChudnovskyBinarySplitting)))
            }
        );
    }

    group.finish();
}

fn benchmark_borwein_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("Borwein - Scaling");

    let digit_counts = [100, 500, 1000, 2000];

    for &digits in &digit_counts {
        group.bench_with_input(
            BenchmarkId::new("Borwein", digits),
            &digits,
            |b, &digits| {
                b.iter(|| compute_pi(black_box(digits), black_box(PiAlgorithm::BorweinQuadratic)))
            }
        );
    }

    group.finish();
}

fn benchmark_comparison_1000(c: &mut Criterion) {
    let mut group = c.benchmark_group("Algorithm Comparison - 1000 digits");

    let algorithms = [
        (PiAlgorithm::ChudnovskyBinarySplitting, "BinarySplitting"),
        (PiAlgorithm::BorweinQuadratic, "Borwein"),
        (PiAlgorithm::MachinFormula, "Machin"),
    ];

    for (algorithm, name) in algorithms {
        group.bench_function(name, |b| {
            b.iter(|| compute_pi(black_box(1000), black_box(algorithm)))
        });
    }

    group.finish();
}

fn benchmark_precision_impact(c: &mut Criterion) {
    let mut group = c.benchmark_group("Precision Impact - Binary Splitting");

    // Тестируем влияние точности на производительность
    let precisions = [
        (10, "10_digits"),
        (50, "50_digits"),
        (100, "100_digits"),
        (500, "500_digits"),
        (1000, "1000_digits"),
        (2000, "2000_digits"),
    ];

    for (digits, name) in precisions {
        group.bench_function(name, |b| {
            b.iter(|| compute_pi(black_box(digits), black_box(PiAlgorithm::ChudnovskyBinarySplitting)))
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    benchmark_algorithms_small,
    benchmark_algorithms_medium,
    benchmark_binary_splitting_scaling,
    benchmark_borwein_scaling,
    benchmark_comparison_1000,
    benchmark_precision_impact
);
criterion_main!(benches);
