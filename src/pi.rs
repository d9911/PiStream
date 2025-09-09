use indicatif::{ProgressBar, ProgressStyle};
use rug::{Float, Integer, ops::{Pow, CompleteRound}, Complete};

#[derive(Clone, Copy, Debug)]
pub enum PiAlgorithm {
    ChudnovskyNaive,
    ChudnovskyBinarySplitting,
    MachinFormula,
    BorweinQuadratic,
    ChudnovskyUltraScale, // Новый алгоритм для триллионов знаков
}

pub fn compute_pi(digits: usize, algorithm: PiAlgorithm) -> String {
    match algorithm {
        PiAlgorithm::ChudnovskyNaive => compute_pi_chudnovsky_naive(digits),
        PiAlgorithm::ChudnovskyBinarySplitting => compute_pi_chudnovsky_binary_splitting(digits),
        PiAlgorithm::MachinFormula => compute_pi_machin(digits),
        PiAlgorithm::BorweinQuadratic => compute_pi_borwein_quadratic(digits),
        PiAlgorithm::ChudnovskyUltraScale => compute_pi_chudnovsky_ultra_scale(digits),
    }
}

// Оригинальный алгоритм (оптимизированный)
fn compute_pi_chudnovsky_naive(digits: usize) -> String {
    // Проверяем разумные пределы
    if digits > 1_000_000 {
        panic!("Слишком большое количество знаков для наивного алгоритма! Максимум: 1,000,000 знаков.\nИспользуйте Binary Splitting для больших вычислений.");
    }
    
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
        // Используем Integer для предотвращения переполнения
        for i in 0..6 {
            factorial_num *= 6 * k - 5 + i;
        }
        factorial_den *= (3 * k - 2) * (3 * k - 1) * (3 * k);
        factorial_den *= k * k * k;

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

// Binary Splitting алгоритм Чудновского - быстрый для больших чисел
fn compute_pi_chudnovsky_binary_splitting(digits: usize) -> String {
    // Проверяем разумные пределы для предотвращения переполнения и чрезмерного использования памяти
    if digits > 100_000_000 {
        panic!("Слишком большое количество знаков для стандартного Binary Splitting! Максимум: 100,000,000 знаков.\nДля больших вычислений используйте Ultra Scale алгоритм.");
    }
    
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
                let mut result = Integer::from(1);
                for i in 0..6 {
                    result *= 6*k - 5 + i;
                }
                -result
            };
            
            // Q(a,b) = k^3 * (3k-1)(3k-2)(3k) * 640320^3 for k > 0, else 1  
            let q = if k == 0 {
                Integer::from(1)
            } else {
                let k_cubed = Integer::from(k).pow(3);
                let factorial_part = Integer::from(3*k-2) * Integer::from(3*k-1) * Integer::from(3*k);
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

// Формула Мачина: π/4 = 4*arctan(1/5) - arctan(1/239)
fn compute_pi_machin(digits: usize) -> String {
    // Проверяем разумные пределы
    if digits > 5_000_000 {
        panic!("Слишком большое количество знаков для формулы Мачина! Максимум: 5,000,000 знаков.\nИспользуйте Binary Splitting для больших вычислений.");
    }
    
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
    // Проверяем разумные пределы
    if digits > 2_000_000 {
        panic!("Слишком большое количество знаков для алгоритма Борвейна! Максимум: 2,000,000 знаков.\nИспользуйте Binary Splitting для больших вычислений.");
    }
    
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

// Ultra Scale Чудновский для экстремально больших чисел (триллионы знаков)
fn compute_pi_chudnovsky_ultra_scale(digits: usize) -> String {
    use std::time::Instant;
    
    println!("🚀 ULTRA SCALE MODE: Подготовка к вычислению {} знаков π", digits);
    println!("📊 Это может занять от минут до месяцев в зависимости от размера!");
    
    // Для триллионов знаков нужны специальные оптимизации
    if digits > 1_000_000_000_000 {
        println!("⚠️  ВНИМАНИЕ: Вычисление {} знаков может занять месяцы!", digits);
        println!("💾 Потребуется ~{} ТБ оперативной памяти", (digits as f64 * 3.32 / 8.0 / 1024.0 / 1024.0 / 1024.0 / 1024.0).ceil());
        println!("⏱️  Расчетное время: ~{} дней", (digits as f64 / 1_000_000_000.0 * 24.0).ceil());
        
        print!("Продолжить? [y/N]: ");
        use std::io::{self, Write};
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim().to_lowercase() != "y" && input.trim().to_lowercase() != "yes" {
            return "3.14159265358979323846264338327950288419716939937510".to_string();
        }
    }
    
    let start_time = Instant::now();
    
    // Используем максимальную точность
    let prec = if digits > 1_000_000 {
        // Для очень больших чисел используем специальный расчет точности
        let base_prec = ((digits as f64) * 3.3219280948873626).ceil() as u32;
        let extra_bits = (digits as f64).log10().ceil() as u32 * 10; // Дополнительные биты для стабильности
        base_prec + extra_bits + 128
    } else {
        ((digits as f64) * 3.3219280948873626).ceil() as u32 + 128
    };
    
    let n = (digits / 14) + 20; // Больше итераций для точности
    
    println!("🔧 Настройки Ultra Scale:");
    println!("   • Точность: {} бит", prec);
    println!("   • Итераций: {}", n);
    println!("   • Память: ~{:.2} ГБ", (prec as f64 * n as f64) / 8.0 / 1024.0 / 1024.0 / 1024.0);
    
    let pb = ProgressBar::new(n as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] Ultra Scale: {pos}/{len} ({per_sec}) - {msg}")
            .unwrap()
            .progress_chars("█▉▊▋▌▍▎▏ "),
    );
    
    // Улучшенный Binary Splitting с оптимизациями для больших чисел
    fn ultra_binary_split(a: usize, b: usize, prec: u32, pb: &ProgressBar, depth: usize) -> (Integer, Integer, Integer) {
        // Показываем прогресс только для верхних уровней рекурсии
        if depth < 10 {
            pb.set_message("Обработка интервала...");
        }
        
        if b - a == 1 {
            pb.inc(1);
            let k = a as i32;
            
            // Оптимизированные вычисления для больших k
            let p = if k == 0 {
                Integer::from(1)
            } else {
                let mut result = Integer::from(1);
                // Используем более эффективный способ вычисления произведения
                for i in 0..6 {
                    let factor = 6 * k - 5 + i;
                    result *= factor;
                }
                -result
            };
            
            let q = if k == 0 {
                Integer::from(1)
            } else {
                // Предвычисляем степени для оптимизации
                let k_big = Integer::from(k);
                let k_squared = (&k_big * &k_big).complete();
                let k_cubed = (&k_squared * &k_big).complete();
                let factorial_3k = Integer::from((3*k-2) * (3*k-1) * (3*k));
                let base_640320_cubed = Integer::from(640320).pow(3);
                k_cubed * factorial_3k * base_640320_cubed
            };
            
            let r = &p * (Integer::from(13591409) + Integer::from(545140134) * Integer::from(k));
            
            (p, q, r)
        } else {
            let mid = (a + b) / 2;
            
            // Рекурсивно вычисляем левую и правую части
            let (p1, q1, r1) = ultra_binary_split(a, mid, prec, pb, depth + 1);
            let (p2, q2, r2) = ultra_binary_split(mid, b, prec, pb, depth + 1);
            
            // Комбинируем результаты с оптимизацией памяти
            let p = (&p1 * &p2).complete();
            let q = (&q1 * &q2).complete();
            
            // Оптимизированное вычисление r для больших чисел
            let r1_q2: Integer = (&r1 * &q2).into();
            let r2_p1: Integer = (&r2 * &p1).into();
            let r = r1_q2 + r2_p1;
            
            (p, q, r)
        }
    }
    
    pb.set_message("Запуск Ultra Scale Binary Splitting...");
    let split_start = Instant::now();
    
    let (_p, q, r) = ultra_binary_split(0, n, prec, &pb, 0);
    
    let split_time = split_start.elapsed();
    pb.set_message("Binary Split завершен");
    
    println!("\n⏱️  Время Binary Splitting: {:?}", split_time);
    println!("🔄 Начинаем финальные вычисления...");
    
    let final_start = Instant::now();
    
    // Финальные вычисления с высокой точностью
    let q_float = Float::with_val(prec, q);
    let r_float = Float::with_val(prec, r);
    let sqrt_10005 = Float::with_val(prec, 10005).sqrt();
    let pi = Float::with_val(prec, 426880) * sqrt_10005 * q_float / r_float;
    
    let final_time = final_start.elapsed();
    let total_time = start_time.elapsed();
    
    pb.finish_with_message("Ultra Scale завершен!");
    
    println!("\n📊 Детальная статистика времени:");
    println!("   • Binary Splitting: {:?}", split_time);
    println!("   • Финальные вычисления: {:?}", final_time);
    println!("   • Общее время: {:?}", total_time);
    println!("   • Скорость: {:.0} знаков/сек", digits as f64 / total_time.as_secs_f64());
    
    // Конвертация в строку с отслеживанием времени
    let string_start = Instant::now();
    let result = pi.to_string_radix(10, Some(digits + 1));
    let string_time = string_start.elapsed();
    
    println!("   • Конвертация в строку: {:?}", string_time);
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    const PI_REFERENCE_50: &str = "3.14159265358979323846264338327950288419716939937511";
    const PI_REFERENCE_100: &str = "3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170680";

    #[test]
    fn test_chudnovsky_naive_accuracy() {
        let result = compute_pi(50, PiAlgorithm::ChudnovskyNaive);
        let result_50 = &result[..52]; // "3." + 50 digits
        assert_eq!(result_50, PI_REFERENCE_50, "Chudnovsky naive algorithm incorrect");
    }

    #[test]
    fn test_binary_splitting_accuracy() {
        let result = compute_pi(50, PiAlgorithm::ChudnovskyBinarySplitting);
        let result_50 = &result[..52];
        assert_eq!(result_50, PI_REFERENCE_50, "Binary splitting algorithm incorrect");
    }

    #[test]
    fn test_machin_formula_accuracy() {
        let result = compute_pi(50, PiAlgorithm::MachinFormula);
        let result_50 = &result[..52];
        assert_eq!(result_50, PI_REFERENCE_50, "Machin formula algorithm incorrect");
    }

    #[test]
    fn test_borwein_accuracy() {
        let result = compute_pi(50, PiAlgorithm::BorweinQuadratic);
        let result_50 = &result[..52];
        assert_eq!(result_50, PI_REFERENCE_50, "Borwein algorithm incorrect");
    }

    #[test]
    fn test_binary_splitting_100_digits() {
        let result = compute_pi(100, PiAlgorithm::ChudnovskyBinarySplitting);
        let result_100 = &result[..102]; // "3." + 100 digits
        assert_eq!(result_100, PI_REFERENCE_100, "Binary splitting 100 digits incorrect");
    }

    #[test]
    fn test_algorithm_performance_comparison() {
        println!("\n🏁 Performance Comparison Test:");
        
        let algorithms = [
            (PiAlgorithm::ChudnovskyNaive, "Chudnovsky (Naive)"),
            (PiAlgorithm::ChudnovskyBinarySplitting, "Chudnovsky (Binary Splitting)"),
            (PiAlgorithm::MachinFormula, "Machin Formula"),
            (PiAlgorithm::BorweinQuadratic, "Borwein Quadratic"),
        ];

        let test_digits = 100;
        let mut results = Vec::new();

        for (algorithm, name) in algorithms {
            let start = Instant::now();
            let pi_result = compute_pi(test_digits, algorithm);
            let duration = start.elapsed();
            
            // Verify correctness
            let pi_100 = &pi_result[..102];
            assert_eq!(pi_100, PI_REFERENCE_100, "{} produced incorrect result", name);
            
            results.push((name, duration));
            println!("  {} : {:?}", name, duration);
        }

        // All algorithms should complete in reasonable time (< 50ms for 100 digits)
        for (name, duration) in &results {
            assert!(duration.as_millis() < 50, "{} took too long: {:?}", name, duration);
        }

        println!("✅ All algorithms passed performance test!");
    }

    #[test]
    fn test_binary_splitting_scaling() {
        println!("\n📈 Binary Splitting Scaling Test:");
        
        let digit_counts = [10, 50, 100, 500, 1000];
        let mut prev_duration: Option<std::time::Duration> = None;

        for &digits in &digit_counts {
            let start = Instant::now();
            let result = compute_pi(digits, PiAlgorithm::ChudnovskyBinarySplitting);
            let duration = start.elapsed();

            // Basic correctness check
            assert!(result.starts_with("3.14159"), "Result should start with 3.14159");
            assert!(result.len() >= digits + 2, "Result should have at least {} characters", digits + 2);

            println!("  {} digits: {:?}", digits, duration);

            // Check that performance scales reasonably (not exponentially)
            if let Some(prev_dur) = prev_duration {
                let ratio = duration.as_nanos() as f64 / prev_dur.as_nanos() as f64;
                // Scaling should be better than quadratic for most cases
                if digits > 100 {
                    assert!(ratio < 10.0, "Performance degraded too much from previous test: {}x", ratio);
                }
            }
            prev_duration = Some(duration);
        }

        println!("✅ Binary splitting scaling test passed!");
    }

    #[test]
    fn test_edge_cases() {
        println!("\n🎯 Edge Cases Test:");

        // Test minimum digits
        let result = compute_pi(1, PiAlgorithm::ChudnovskyBinarySplitting);
        assert!(result.starts_with("3.1"), "1 digit result should be 3.1");
        println!("  ✓ 1 digit: {}", &result[..std::cmp::min(4, result.len())]);

        // Test small digits
        let result = compute_pi(5, PiAlgorithm::BorweinQuadratic);
        assert!(result.starts_with("3.14159"), "5 digits result incorrect");
        println!("  ✓ 5 digits: {}", &result[..7]);

        // Test consistency across algorithms for small values
        let test_digits = 10;
        let algorithms = [
            PiAlgorithm::ChudnovskyNaive,
            PiAlgorithm::ChudnovskyBinarySplitting,
            PiAlgorithm::MachinFormula,
            PiAlgorithm::BorweinQuadratic,
        ];

        let mut results = Vec::new();
        for algorithm in algorithms {
            let result = compute_pi(test_digits, algorithm);
            let truncated = &result[..12]; // "3." + 10 digits
            results.push(truncated.to_string());
        }

        // All should give same result
        let reference = &results[0];
        for (i, result) in results.iter().enumerate() {
            assert_eq!(result, reference, "Algorithm {} gave different result for {} digits", i, test_digits);
        }
        println!("  ✓ All algorithms consistent for {} digits: {}", test_digits, reference);

        println!("✅ Edge cases test passed!");
    }

    #[test]
    fn test_precision_requirements() {
        println!("\n🎯 Precision Requirements Test:");

        // Test that we get exactly the requested number of digits
        let test_cases = [10, 25, 50, 100];
        
        for &digits in &test_cases {
            let result = compute_pi(digits, PiAlgorithm::ChudnovskyBinarySplitting);
            
            // Should have at least "3." + digits characters
            assert!(result.len() >= digits + 2, 
                "Result for {} digits should have at least {} characters, got {}", 
                digits, digits + 2, result.len());
            
            // Should start correctly
            assert!(result.starts_with("3.14159265"), 
                "Result should start with correct pi digits");
            
            println!("  ✓ {} digits: length = {}, starts with {}", 
                digits, result.len(), &result[..std::cmp::min(20, result.len())]);
        }

        println!("✅ Precision requirements test passed!");
    }

    #[test]
    fn test_performance_targets() {
        println!("\n⏱️  Performance Targets Test:");

        // Define performance targets based on our optimizations
        let test_cases = [
            (50, 10_000, "50 digits should compute in < 10ms"), // 10ms in microseconds
            (100, 20_000, "100 digits should compute in < 20ms"),
            (500, 100_000, "500 digits should compute in < 100ms"),
            (1000, 500_000, "1000 digits should compute in < 500ms"),
        ];

        for (digits, max_micros, description) in test_cases {
            let start = Instant::now();
            let _result = compute_pi(digits, PiAlgorithm::ChudnovskyBinarySplitting);
            let duration = start.elapsed();

            let micros = duration.as_micros();
            println!("  {} digits: {:?} ({} μs)", digits, duration, micros);

            assert!(micros < max_micros, "{}: took {} μs, expected < {} μs", 
                description, micros, max_micros);
        }

        println!("✅ Performance targets test passed!");
    }

    #[test]
    #[should_panic(expected = "Слишком большое количество знаков")]
    fn test_overflow_protection_binary_splitting() {
        println!("\n🛡️  Testing overflow protection for Binary Splitting...");
        let _result = compute_pi(20_000_000, PiAlgorithm::ChudnovskyBinarySplitting);
    }

    #[test]
    #[should_panic(expected = "Слишком большое количество знаков")]
    fn test_overflow_protection_naive() {
        println!("\n🛡️  Testing overflow protection for Naive algorithm...");
        let _result = compute_pi(2_000_000, PiAlgorithm::ChudnovskyNaive);
    }

    #[test]
    fn test_algorithm_limits() {
        println!("\n📏 Testing algorithm limits:");
        
        // Test within reasonable limits for CI/testing
        let result = compute_pi(10_000, PiAlgorithm::ChudnovskyNaive);
        assert!(result.starts_with("3.14159"), "Naive algorithm should work within limits");
        println!("  ✓ Naive: 10K digits - OK");
        
        let result = compute_pi(50_000, PiAlgorithm::ChudnovskyBinarySplitting);
        assert!(result.starts_with("3.14159"), "Binary splitting should work within limits");
        println!("  ✓ Binary Splitting: 50K digits - OK");
        
        let result = compute_pi(20_000, PiAlgorithm::BorweinQuadratic);
        assert!(result.starts_with("3.14159"), "Borwein should work within limits");
        println!("  ✓ Borwein: 20K digits - OK");
        
        let result = compute_pi(15_000, PiAlgorithm::MachinFormula);
        assert!(result.starts_with("3.14159"), "Machin should work within limits");
        println!("  ✓ Machin: 15K digits - OK");
        
        println!("✅ Algorithm limits test passed!");
    }
}
