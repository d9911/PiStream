use crate::domain::value_objects::Precision;
use indicatif::{ProgressBar, ProgressStyle};
use rug::{
    ops::{CompleteRound, Pow},
    Float,
};

pub fn compute_pi_machin(digits: usize, precision: Precision) -> String {
    // Проверяем разумные пределы
    if digits > 5_000_000 {
        panic!("Слишком большое количество знаков для формулы Мачина! Максимум: 5,000,000 знаков.\nИспользуйте Binary Splitting для больших вычислений.");
    }

    let prec = precision.bits();

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );

    pb.set_message("Computing Machin's formula...");

    // Вычисляем arctan(1/5) и arctan(1/239) с помощью ряда Тейлора
    fn arctan_series(x_inv: u32, prec: u32, terms: usize, digits: usize) -> Float {
        let mut result = Float::with_val(prec, 0);
        let x_inv_float = Float::with_val(prec, x_inv);
        let x_inv_squared = (&x_inv_float * &x_inv_float).complete(prec);

        let mut term = Float::with_val(prec, 1) / &x_inv_float; // 1/x
        let mut power_of_x_inv_squared = Float::with_val(prec, 1);

        for n in 0..terms {
            let coeff = if n % 2 == 0 { 1 } else { -1 };
            let current_term = Float::with_val(prec, coeff) * &term / (2 * n + 1);
            result += &current_term;

            // Следующий член: умножаем на 1/x^2
            let temp_mul = (&power_of_x_inv_squared * &x_inv_squared).complete(prec);
            power_of_x_inv_squared = temp_mul;
            let temp_div = (&x_inv_float * &power_of_x_inv_squared).complete(prec);
            term = Float::with_val(prec, 1) / temp_div;

            // Проверяем сходимость
            let threshold = Float::with_val(64, 10).pow(-(digits as i32 + 10));
            if current_term.abs() < threshold {
                break;
            }
        }
        result
    }

    let terms = digits + 50; // достаточно терминов для сходимости
    let arctan_1_5 = arctan_series(5, prec, terms, digits);
    let arctan_1_239 = arctan_series(239, prec, terms, digits);

    // π = 4 * (4*arctan(1/5) - arctan(1/239))
    let pi = Float::with_val(prec, 4) * (Float::with_val(prec, 4) * arctan_1_5 - arctan_1_239);

    pb.finish_with_message("Machin's formula complete");
    pi.to_string_radix(10, Some(digits + 1))
}
