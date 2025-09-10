use crate::domain::value_objects::Precision;
use indicatif::{ProgressBar, ProgressStyle};
use rug::{ops::Pow, Float, Integer};

pub fn compute_pi_chudnovsky_binary_splitting(digits: usize, precision: Precision) -> String {
    // Проверяем разумные пределы для предотвращения переполнения и чрезмерного использования памяти
    if digits > 100_000_000 {
        panic!("Слишком большое количество знаков для стандартного Binary Splitting! Максимум: 100,000,000 знаков.\nДля больших вычислений используйте Ultra Scale алгоритм.");
    }

    let prec = precision.bits();
    let n = (digits / 14) + 5;

    let pb = ProgressBar::new(n as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] Binary Splitting: {pos}/{len} - {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );

    fn binary_split(
        a: usize,
        b: usize,
        prec: u32,
        pb: &ProgressBar,
    ) -> (Integer, Integer, Integer) {
        if b - a == 1 {
            pb.inc(1);
            let k = a as i32;

            // P(a,b) = -(6k-5)(6k-4)(6k-3)(6k-2)(6k-1)(6k) for k > 0, else 1
            let p = if k == 0 {
                Integer::from(1)
            } else {
                let mut result = Integer::from(1);
                for i in 0..6 {
                    result *= 6 * k - 5 + i;
                }
                -result
            };

            // Q(a,b) = k^3 * (3k-1)(3k-2)(3k) * 640320^3 for k > 0, else 1
            let q = if k == 0 {
                Integer::from(1)
            } else {
                let k_cubed = Integer::from(k).pow(3);
                let factorial_part =
                    Integer::from(3 * k - 2) * Integer::from(3 * k - 1) * Integer::from(3 * k);
                let base_power = Integer::from(640320).pow(3);
                k_cubed * factorial_part * base_power
            };

            // R(a,b) = P(a,b) * (13591409 + 545140134*k)
            let r = &p * (Integer::from(13591409) + Integer::from(545140134) * Integer::from(k));

            (p, q, r)
        } else {
            let mid = (a + b) / 2;
            let (p1, q1, r1) = binary_split(a, mid, prec, pb);
            let (p2, q2, r2) = binary_split(mid, b, prec, pb);

            let p = (&p1 * &p2).into();
            let q = (&q1 * &q2).into();
            let r_temp1: Integer = (&r1 * &q2).into();
            let r_temp2: Integer = (&r2 * &p1).into();
            let r = r_temp1 + r_temp2;

            (p, q, r)
        }
    }

    pb.set_message("Computing binary split...");
    let (_p, q, r) = binary_split(0, n, prec, &pb);

    pb.set_message("Computing final result...");

    // pi = 426880 * sqrt(10005) * q / r
    let q_float = Float::with_val(prec, q);
    let r_float = Float::with_val(prec, r);
    let sqrt_10005 = Float::with_val(prec, 10005).sqrt();
    let pi = Float::with_val(prec, 426880) * sqrt_10005 * q_float / r_float;

    pb.finish_with_message("Binary splitting complete");
    pi.to_string_radix(10, Some(digits + 1))
}
