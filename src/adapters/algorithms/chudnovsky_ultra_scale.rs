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
    let prec = precision.bits();

    // Безопасное вычисление количества итераций с защитой от переполнения
    let n = if digits > usize::MAX / 2 {
        // Для экстремально больших чисел используем максимально безопасное значение
        usize::MAX / 100
    } else {
        (digits / 14).saturating_add(20) // Больше итераций для точности
    };

    println!("🔧 Настройки Ultra Scale:");
    println!("   • Точность: {} бит", prec);
    println!("   • Итераций: {}", n);
    println!(
        "   • Память: ~{:.2} ГБ",
        (prec as f64 * n as f64) / 8.0 / 1024.0 / 1024.0 / 1024.0
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
    println!(
        "   • Скорость: {:.0} знаков/сек",
        digits as f64 / total_time.as_secs_f64()
    );

    // Конвертация в строку с отслеживанием времени
    let string_start = Instant::now();
    let result = pi.to_string_radix(10, Some(digits + 1));
    let string_time = string_start.elapsed();

    println!("   • Конвертация в строку: {:?}", string_time);

    result
}
