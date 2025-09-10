use crate::domain::value_objects::Precision;
use indicatif::{ProgressBar, ProgressStyle};
use rug::{ops::Pow, Float, Integer};

pub fn compute_pi_chudnovsky_naive(digits: usize, precision: Precision) -> String {
    // Проверяем разумные пределы
    if digits > 1_000_000 {
        panic!("Слишком большое количество знаков для наивного алгоритма! Максимум: 1,000,000 знаков.\nИспользуйте Binary Splitting для больших вычислений.");
    }

    let prec = precision.bits();

    // Initialize sum
    let mut sum = Float::with_val(prec, 0);

    // Constants for Chudnovsky algorithm - предвычисляем константы
    let mut a = Float::with_val(prec, 13591409);
    let mut factorial_num = Integer::from(1);
    let mut factorial_den = Integer::from(1);
    let mut power_of_640320_cubed = Integer::from(1);
    let base_640320_cubed = Integer::from(640320).pow(3);

    // Estimate number of iterations (more accurate)
    let max_k = (digits / 14) + 5;
    let pb = ProgressBar::new(max_k as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta}) - {msg}",
            )
            .unwrap()
            .progress_chars("#>-"),
    );

    let mut k = 0;
    loop {
        // Compute term: (-1)^k * a / (factorial_ratio * 640320^(3k))
        let sign = if k % 2 == 0 { 1 } else { -1 };

        // Используем Integer для точных вычислений, потом конвертируем в Float
        let denominator_int: Integer = (&factorial_den * &power_of_640320_cubed).into();
        let denominator = Float::with_val(prec, denominator_int);
        let numerator = Float::with_val(prec, sign) * &a * Float::with_val(prec, &factorial_num);
        let term = numerator / denominator;
        sum += &term;

        // Check convergence more efficiently
        if k > 5 {
            let term_magnitude = term.abs();
            let threshold = Float::with_val(64, 10).pow(-(digits as i32 + 5));
            if term_magnitude < threshold {
                pb.set_message("Converged");
                break;
            }
        }

        // Update for next k
        k += 1;
        a += 545140134;

        // Более эффективное обновление факториалов с защитой от переполнения
        // (6k)! / (3k)! / k!^3 = (6k-5)(6k-4)(6k-3)(6k-2)(6k-1)(6k) / ((3k-2)(3k-1)(3k) * k^3)
        // Используем Integer для предотвращения переполнения
        for i in 0..6 {
            let factor = Integer::from(6 * k - 5 + i);
            factorial_num *= &factor;
        }
        let factor_3k_2 = Integer::from(3 * k - 2);
        let factor_3k_1 = Integer::from(3 * k - 1);
        let factor_3k = Integer::from(3 * k);
        let factor_k_cubed = Integer::from(k).pow(3);

        factorial_den *= &factor_3k_2;
        factorial_den *= &factor_3k_1;
        factorial_den *= &factor_3k;
        factorial_den *= &factor_k_cubed;

        // Update power of 640320^3
        power_of_640320_cubed *= &base_640320_cubed;

        pb.set_position(k as u64);
        if k >= max_k {
            pb.println("Warning: Reached maximum iterations, precision may be insufficient.");
            break;
        }
    }

    pb.finish_with_message("Computation complete");

    // Compute pi using Chudnovsky formula: pi = 426880 * sqrt(10005) / sum
    let sqrt_10005 = Float::with_val(prec, 10005).sqrt();
    let pi = Float::with_val(prec, 426880) * sqrt_10005 / sum;

    // Convert to string with specified digits
    pi.to_string_radix(10, Some(digits + 1))
}
