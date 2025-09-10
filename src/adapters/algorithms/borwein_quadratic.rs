use crate::domain::value_objects::Precision;
use indicatif::{ProgressBar, ProgressStyle};
use rug::{
    ops::{CompleteRound, Pow},
    Float,
};

pub fn compute_pi_borwein_quadratic(digits: usize, precision: Precision) -> String {
    // Проверяем разумные пределы
    if digits > 2_000_000 {
        panic!("Слишком большое количество знаков для алгоритма Борвейна! Максимум: 2,000,000 знаков.\nИспользуйте Binary Splitting для больших вычислений.");
    }

    let prec = precision.bits();

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg} - iteration {pos}")
            .unwrap(),
    );

    pb.set_message("Computing Borwein quadratic...");

    let mut a = Float::with_val(prec, 1);
    let mut b = Float::with_val(prec, 2).sqrt().recip(); // 1/sqrt(2)
    let mut t = Float::with_val(prec, 0.25); // 1/4
    let mut p = Float::with_val(prec, 1);

    // Оценка количества итераций (каждая удваивает точность)
    let max_iterations = ((digits as f64).log2()).ceil() as usize + 5;

    for i in 0..max_iterations {
        pb.set_position(i as u64);

        let a_plus_b = (&a + &b).complete(prec);
        let a_new = a_plus_b * Float::with_val(prec, 0.5);
        let a_mul_b = (&a * &b).complete(prec);
        let b_new = a_mul_b.sqrt();
        let a_minus_a_new = (&a - &a_new).complete(prec);
        let t_new = &t - &p * a_minus_a_new.pow(2);
        let p_new = Float::with_val(prec, &p * 2);

        // Проверяем сходимость
        let a_minus_b = (&a - &b).complete(prec);
        let threshold = Float::with_val(64, 10).pow(-(digits as i32 + 5));
        if a_minus_b.abs() < threshold {
            pb.set_message("Converged");
            break;
        }

        a = a_new;
        b = b_new;
        t = t_new;
        p = p_new;
    }

    let a_plus_b = (&a + &b).complete(prec);
    let pi: Float = a_plus_b.pow(2) / (Float::with_val(prec, 4) * &t);

    pb.finish_with_message("Borwein quadratic complete");
    pi.to_string_radix(10, Some(digits + 1))
}
