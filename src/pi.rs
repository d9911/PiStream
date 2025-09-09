use indicatif::{ProgressBar, ProgressStyle};
use rug::{Float, Integer, ops::{Pow, CompleteRound}};

#[derive(Clone, Copy, Debug)]
pub enum PiAlgorithm {
    ChudnovskyNaive,
    ChudnovskyBinarySplitting,
    MachinFormula,
    BorweinQuadratic,
}

pub fn compute_pi(digits: usize, algorithm: PiAlgorithm) -> String {
    match algorithm {
        PiAlgorithm::ChudnovskyNaive => compute_pi_chudnovsky_naive(digits),
        PiAlgorithm::ChudnovskyBinarySplitting => compute_pi_chudnovsky_binary_splitting(digits),
        PiAlgorithm::MachinFormula => compute_pi_machin(digits),
        PiAlgorithm::BorweinQuadratic => compute_pi_borwein_quadratic(digits),
    }
}

// Оригинальный алгоритм (оптимизированный)
fn compute_pi_chudnovsky_naive(digits: usize) -> String {
    // Calculate required precision in bits (approximately digits * log2(10))
    let prec = ((digits as f64) * 3.3219280948873626).ceil() as u32 + 32; // больше запаса для точности

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

        // Более эффективное обновление факториалов
        // (6k)! / (3k)! / k!^3 = (6k-5)(6k-4)(6k-3)(6k-2)(6k-1)(6k) / ((3k-2)(3k-1)(3k) * k^3)
        factorial_num *= (6 * k - 5) * (6 * k - 4) * (6 * k - 3) * (6 * k - 2) * (6 * k - 1) * (6 * k);
        factorial_den *= (3 * k - 2) * (3 * k - 1) * (3 * k) * k * k * k;

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

// Binary Splitting алгоритм Чудновского - самый быстрый для больших чисел
fn compute_pi_chudnovsky_binary_splitting(digits: usize) -> String {
    let prec = ((digits as f64) * 3.3219280948873626).ceil() as u32 + 64;
    let n = (digits / 14) + 5;

    let pb = ProgressBar::new(n as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] Binary Splitting: {pos}/{len} - {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );

    fn binary_split(a: usize, b: usize, prec: u32, pb: &ProgressBar) -> (Integer, Integer, Integer) {
        if b - a == 1 {
            pb.inc(1);
            let k = a as i32;
            
            // P(a,b) = -(6k-5)(6k-4)(6k-3)(6k-2)(6k-1)(6k) for k > 0, else 1
            let p = if k == 0 {
                Integer::from(1)
            } else {
                -Integer::from((6*k-5) * (6*k-4) * (6*k-3) * (6*k-2) * (6*k-1) * (6*k))
            };
            
            // Q(a,b) = k^3 * (3k-1)(3k-2)(3k) * 640320^3 for k > 0, else 1  
            let q = if k == 0 {
                Integer::from(1)
            } else {
                Integer::from(k * k * k) * 
                Integer::from((3*k-2) * (3*k-1) * (3*k)) * 
                Integer::from(640320).pow(3)
            };
            
            // R(a,b) = P(a,b) * (13591409 + 545140134*k)
            let r = &p * Integer::from(13591409 + 545140134 * k);
            
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

// Формула Мачина: π/4 = 4*arctan(1/5) - arctan(1/239)
fn compute_pi_machin(digits: usize) -> String {
    let prec = ((digits as f64) * 3.3219280948873626).ceil() as u32 + 32;
    
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner()
        .template("{spinner:.green} {msg}")
        .unwrap());
    
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

// Алгоритм Борвейна (квадратичная сходимость)
fn compute_pi_borwein_quadratic(digits: usize) -> String {
    let prec = ((digits as f64) * 3.3219280948873626).ceil() as u32 + 32;
    
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner()
        .template("{spinner:.green} {msg} - iteration {pos}")
        .unwrap());
    
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
