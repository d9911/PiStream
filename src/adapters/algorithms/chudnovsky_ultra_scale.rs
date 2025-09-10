use crate::domain::value_objects::Precision;
use indicatif::{ProgressBar, ProgressStyle};
use rug::{
    ops::Pow,
    Complete, Float, Integer,
};
use std::time::Instant;

pub fn compute_pi_chudnovsky_ultra_scale(digits: usize, precision: Precision) -> String {
    println!(
        "🚀 ULTRA SCALE MODE: Подготовка к вычислению {} знаков π",
        digits
    );
    println!("📊 Это может занять от минут до месяцев в зависимости от размера!");

    // Для триллионов знаков нужны специальные оптимизации
    if digits > 1_000_000_000_000 {
        println!(
            "⚠️  ВНИМАНИЕ: Вычисление {} знаков может занять месяцы!",
            digits
        );
        println!(
            "💾 Потребуется ~{} ТБ оперативной памяти",
            (digits as f64 * 3.32 / 8.0 / 1024.0 / 1024.0 / 1024.0 / 1024.0).ceil()
        );
        println!(
            "⏱️  Расчетное время: ~{} дней",
            (digits as f64 / 1_000_000_000.0 * 24.0).ceil()
        );
    }

    // Дополнительная проверка для очень больших чисел
    if digits > 500_000_000 {
        println!("⚠️  ВНИМАНИЕ: Вычисление {} знаков может занять очень много времени и памяти!", digits);
        println!("⚠️  Рекомендуется использовать меньшее количество знаков для стабильной работы.");

        print!("Продолжить? [y/N]: ");
        use std::io::{self, Write};
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim().to_lowercase() != "y" && input.trim().to_lowercase() != "yes" {
            return "3.14159265358979323846264338327950288419716939937510".to_string();
        }
    }

    // Ограничение для предотвращения экстремальных вычислений
    if digits > 1_000_000_000 {
        println!("🚫 ОШИБКА: Слишком большое количество знаков: {}", digits);
        println!("🚫 Максимально поддерживаемое количество: 1,000,000,000 знаков");
        println!("🚫 Для больших вычислений используйте специализированные системы");
        return "3.14159265358979323846264338327950288419716939937510".to_string();
    }

    let start_time = Instant::now();
    let prec = precision.bits();
    // Добавляем дополнительную точность для очень больших чисел
    let extra_precision = if digits > 200_000_000 {
        2000  // Очень большая дополнительная точность для экстремальных чисел
    } else if digits > 100_000_000 {
        1000  // Большая дополнительная точность для больших чисел
    } else if digits > 50_000_000 {
        500   // Средняя дополнительная точность
    } else {
        0
    };
    let final_prec = prec + extra_precision;

    // Безопасное вычисление количества итераций с защитой от переполнения
    let n = if digits > usize::MAX / 2 {
        // Для экстремально больших чисел используем максимально безопасное значение
        usize::MAX / 100
    } else {
        (digits / 14).saturating_add(20) // Больше итераций для точности
    };

    println!("🔧 Настройки Ultra Scale:");
    println!("   • Точность: {} бит (базовая: {}, дополнительная: {})", final_prec, prec, extra_precision);
    println!("   • Итераций: {}", n);
    println!(
        "   • Память: ~{:.2} ГБ",
        (final_prec as f64 * n as f64) / 8.0 / 1024.0 / 1024.0 / 1024.0
    );

    let pb = ProgressBar::new(n as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] Ultra Scale: {pos}/{len} ({per_sec}) - {msg}",
            )
            .unwrap()
            .progress_chars("█▉▊▋▌▍▎▏ "),
    );

    // Улучшенный Binary Splitting с оптимизациями для больших чисел
    fn ultra_binary_split(
        a: usize,
        b: usize,
        prec: u32,
        pb: &ProgressBar,
        depth: usize,
    ) -> (Integer, Integer, Integer) {
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
                // Предвычисляем степени для оптимизации с защитой от переполнения
                let k_big = Integer::from(k);
                let k_squared = (&k_big * &k_big).complete();
                let k_cubed = (&k_squared * &k_big).complete();

                // Вычисляем факториальную часть безопасно
                let factorial_3k =
                    Integer::from(3 * k - 2) * Integer::from(3 * k - 1) * Integer::from(3 * k);
                let base_640320_cubed = Integer::from(640320).pow(3);

                // Используем поэтапное умножение для предотвращения переполнения
                let temp1 = (&k_cubed * &factorial_3k).complete();
                (&temp1 * &base_640320_cubed).complete()
            };

            let r = &p * (Integer::from(13591409) + Integer::from(545140134) * Integer::from(k));

            (p, q, r)
        } else {
            let mid = (a + b) / 2;

            // Рекурсивно вычисляем левую и правую части
            let (p1, q1, r1) = ultra_binary_split(a, mid, prec, pb, depth + 1);
            let (p2, q2, r2) = ultra_binary_split(mid, b, prec, pb, depth + 1);

            // Комбинируем результаты с оптимизацией памяти и проверками
            let p = (&p1 * &p2).complete();
            let q = (&q1 * &q2).complete();

            // Оптимизированное вычисление r для больших чисел с проверками
            let r1_q2: Integer = (&r1 * &q2).into();
            let r2_p1: Integer = (&r2 * &p1).into();
            let r = r1_q2 + r2_p1;

            // Проверяем на переполнение
            if p.is_zero() || q.is_zero() {
                panic!("Ошибка: Переполнение в binary splitting! p или q равно нулю.");
            }

            (p, q, r)
        }
    }

    pb.set_message("Запуск Ultra Scale Binary Splitting...");
    let split_start = Instant::now();

    let (_p, q, r) = ultra_binary_split(0, n, final_prec, &pb, 0);

    let split_time = split_start.elapsed();
    pb.set_message("Binary Split завершен");

    println!("\n⏱️  Время Binary Splitting: {:?}", split_time);
    println!("🔄 Начинаем финальные вычисления...");

    let final_start = Instant::now();

    // Финальные вычисления с высокой точностью и проверками
    let q_float = Float::with_val(final_prec, q.clone());
    let r_float = Float::with_val(final_prec, r.clone());

    // Проверяем на нулевые значения
    if r_float.is_zero() {
        panic!("Ошибка: r равно нулю! Возможно переполнение при вычислении.");
    }

    let sqrt_10005 = Float::with_val(final_prec, 10005).sqrt();

    // Вычисляем π поэтапно для предотвращения переполнения
    // Используем логарифмический подход для очень больших чисел
    let numerator = Float::with_val(final_prec, 426880) * sqrt_10005.clone() * q_float.clone();

    // Для очень больших чисел используем более безопасный метод
    let pi = if digits >= 100_000_000 {
        // Безопасный логарифмический подход с промежуточными проверками
        // Разбиваем вычисления на более мелкие части для предотвращения переполнения
        
    // Отладочная информация о промежуточных значениях
    println!("🔍 Отладка промежуточных значений:");
    println!("   q_float: {:?}", q_float);
    println!("   r_float: {:?}", r_float);
    println!("   numerator: {:?}", numerator);
    
    // Проверяем промежуточные значения перед вычислением логарифмов
    if numerator.is_infinite() || numerator.is_nan() {
        panic!("Ошибка: numerator стал бесконечным или NaN! numerator: {:?}", numerator);
    }
    
    if r_float.is_infinite() || r_float.is_nan() {
        panic!("Ошибка: r_float стал бесконечным или NaN! r_float: {:?}", r_float);
    }

        // Используем более безопасный подход с масштабированием
        // Вычисляем логарифмы компонентов отдельно
        let log_426880 = Float::with_val(final_prec, 426880).ln();
        let log_sqrt_10005 = sqrt_10005.ln();
        let log_q = q_float.ln();
        let log_r = r_float.ln();

        // Отладочная информация
        println!("🔍 Отладка логарифмов:");
        println!("   log_426880: {:?}", log_426880);
        println!("   log_sqrt_10005: {:?}", log_sqrt_10005);
        println!("   log_q: {:?}", log_q);
        println!("   log_r: {:?}", log_r);

        // Проверяем каждый логарифм отдельно
        if log_426880.is_infinite() || log_426880.is_nan() ||
           log_sqrt_10005.is_infinite() || log_sqrt_10005.is_nan() ||
           log_q.is_infinite() || log_q.is_nan() ||
           log_r.is_infinite() || log_r.is_nan() {
            // Если даже отдельные логарифмы переполняются, используем альтернативный подход
            println!("⚠️  Предупреждение: Обнаружено переполнение в логарифмах компонентов");
            println!("⚠️  Переключаемся на альтернативный метод вычисления...");
            
            // Альтернативный подход: используем более низкую точность для промежуточных вычислений
            let reduced_prec = final_prec / 2;
            let log_426880_alt = Float::with_val(reduced_prec, 426880).ln();
            let log_sqrt_10005_alt = Float::with_val(reduced_prec, 10005).sqrt().ln();
            let log_q_alt = Float::with_val(reduced_prec, q).ln();
            let log_r_alt = Float::with_val(reduced_prec, r).ln();
            
            // Проверяем альтернативные логарифмы
            if log_426880_alt.is_infinite() || log_426880_alt.is_nan() ||
               log_sqrt_10005_alt.is_infinite() || log_sqrt_10005_alt.is_nan() ||
               log_q_alt.is_infinite() || log_q_alt.is_nan() ||
               log_r_alt.is_infinite() || log_r_alt.is_nan() {
                panic!("Ошибка: Даже с пониженной точностью логарифмы переполняются! log_426880: {:?}, log_sqrt_10005: {:?}, log_q: {:?}, log_r: {:?}", 
                       log_426880_alt, log_sqrt_10005_alt, log_q_alt, log_r_alt);
            }
            
            // Используем альтернативные значения
            let log_numerator = log_426880_alt + log_sqrt_10005_alt + log_q_alt;
            let log_pi = log_numerator - log_r_alt;
            let pi_result = log_pi.clone().exp();
            
            // Проверяем результат альтернативного метода
            if pi_result.is_nan() || pi_result.is_infinite() {
                panic!("Ошибка: Альтернативный метод также дал некорректный результат! log_pi: {:?}, pi_result: {:?}", log_pi, pi_result);
            }
            
            pi_result
        } else {
            // Обычный путь: все логарифмы корректны
            let log_numerator = log_426880 + log_sqrt_10005 + log_q;
            
            // Проверяем финальный логарифм числителя
            if log_numerator.is_infinite() || log_numerator.is_nan() {
                panic!("Ошибка: Финальный log_numerator некорректный! log_numerator: {:?}", log_numerator);
            }

            let log_pi = log_numerator - log_r;
            let pi_result = log_pi.clone().exp();
            
            // Проверяем результат
            if pi_result.is_nan() || pi_result.is_infinite() {
                panic!("Ошибка: Некорректный результат после экспоненты! log_pi: {:?}, pi_result: {:?}", log_pi, pi_result);
            }
            
            pi_result
        }
    } else {
        // Обычный подход для меньших чисел
        println!("🔍 Используем обычный подход для {} знаков", digits);
        println!("   numerator: {:?}", numerator);
        println!("   r_float: {:?}", r_float);
        let result = numerator / r_float;
        println!("   result: {:?}", result);
        result
    };

    // Проверяем результат на валидность
    if pi.is_nan() || pi.is_infinite() {
        panic!("Ошибка: Результат NaN или бесконечность! Возможно переполнение при вычислении.");
    }

    let final_time = final_start.elapsed();
    let total_time = start_time.elapsed();

    pb.finish_with_message("Ultra Scale завершен!");

    println!("\n📊 Детальная статистика времени:");
    println!("   • Binary Splitting: {:?}", split_time);
    println!("   • Финальные вычисления: {:?}", final_time);
    println!("   • Общее время: {:?}", total_time);
    println!(
        "   • Скорость: {:.0} знаков/сек",
        digits as f64 / total_time.as_secs_f64()
    );

    // Конвертация в строку с отслеживанием времени и проверками
    let string_start = Instant::now();
    let result = pi.to_string_radix(10, Some(digits + 1));
    let string_time = string_start.elapsed();

    println!("   • Конвертация в строку: {:?}", string_time);

    // Проверяем результат на валидность
    if result.contains("NaN") || result.contains("Inf") || result.is_empty() {
        panic!("Ошибка: Некорректный результат при конвертации в строку: {}", result);
    }

    result
}
