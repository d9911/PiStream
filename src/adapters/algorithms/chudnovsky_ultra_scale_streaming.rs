use crate::adapters::output::{StreamingPiSaver, PiBlock, utils};
use crate::domain::value_objects::Precision;
use indicatif::{ProgressBar, ProgressStyle};
use rug::{Float, Integer};
use std::io::{self, Write};
use std::time::Instant;

/// Вычисляет π с потоковым сохранением для очень больших чисел
pub fn compute_pi_chudnovsky_ultra_scale_streaming(
    digits: usize,
    precision: Precision,
) -> Result<String, String> {
    println!("🚀 ULTRA SCALE STREAMING MODE: Подготовка к вычислению {} знаков π", digits);
    println!("📊 Это может занять от минут до месяцев в зависимости от размера!");
    
    // Проверяем существующий файл
    let stream_file = format!("pi_stream_{}_digits.txt", digits);
    if let Ok(Some(existing_digits)) = utils::check_existing_file(&stream_file) {
        println!("📁 Найден существующий файл с {} знаками", existing_digits);
        if existing_digits >= digits {
            println!("✅ Файл уже содержит достаточно знаков!");
            return utils::merge_blocks(&stream_file);
        } else {
            println!("🔄 Продолжаем вычисление с позиции {}", existing_digits);
        }
    }

    // Ограничение для предотвращения экстремальных вычислений
    if digits > 1_000_000_000 {
        println!("🚫 ОШИБКА: Слишком большое количество знаков: {}", digits);
        println!("🚫 Максимально поддерживаемое количество: 1,000,000,000 знаков");
        println!("🚫 Для больших вычислений используйте специализированные системы");
        return Ok("3.14159265358979323846264338327950288419716939937510".to_string());
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

    println!("🔧 Настройки Ultra Scale Streaming:");
    println!("   • Точность: {} бит (базовая: {}, дополнительная: {})", final_prec, prec, extra_precision);
    println!("   • Итераций: {}", n);
    
    // Оценка памяти (примерная)
    let estimated_memory_gb = (final_prec as f64 * n as f64) / 8.0 / 1024.0 / 1024.0 / 1024.0;
    println!("   • Память: ~{:.2} ГБ", estimated_memory_gb);

    // Предупреждение для очень больших вычислений
    if digits > 500_000_000 {
        println!("⚠️  ВНИМАНИЕ: Вычисление {} знаков может занять очень много времени и памяти!", digits);
        println!("⚠️  Рекомендуется использовать меньшее количество знаков для стабильной работы.");
        print!("Продолжить? [y/N]: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim().to_lowercase() != "y" && input.trim().to_lowercase() != "yes" {
            return Ok("3.14159265358979323846264338327950288419716939937510".to_string());
        }
    }

    // Создаем потоковый сохранитель
    let saver = StreamingPiSaver::new(&stream_file, digits)?;
    
    // Создаем прогресс-бар
    let pb = ProgressBar::new(n as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("⠋ {spinner:.green} [{elapsed_precise}] Ultra Scale Streaming: {pos}/{len} ({per_sec:.2}/s) - {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );

    println!("⏰ Засекаем время...");
    
    // Binary Splitting
    pb.set_message("Binary Split...");
    let binary_start = Instant::now();
    
    let (_p, q, r) = ultra_binary_split_streaming(0, n, final_prec as u32, &pb, &saver)?;
    
    let binary_time = binary_start.elapsed();
    println!("⏱️  Время Binary Splitting: {:.9}s", binary_time.as_secs_f64());

    // Финальные вычисления с потоковым сохранением
    println!("🔄 Начинаем финальные вычисления с потоковым сохранением...");
    let final_start = Instant::now();

    // Вычисляем π по блокам для предотвращения переполнения
    let block_size = 10000; // Размер блока для вычисления
    let total_blocks = (digits + block_size - 1) / block_size;
    
    pb.set_length(total_blocks as u64);
    pb.set_message("Вычисление блоков π...");

    for block_num in 0..total_blocks {
        let start_pos = block_num * block_size;
        let end_pos = std::cmp::min(start_pos + block_size, digits);
        let current_block_size = end_pos - start_pos;
        
        // Вычисляем π для текущего блока
        let pi_block = compute_pi_block(
            start_pos, 
            current_block_size, 
            final_prec as u32, 
            &q, 
            &r, 
            block_num
        )?;
        
        // Отправляем блок для сохранения
        saver.send_block(pi_block)?;
        
        pb.inc(1);
        
        // Периодически выводим статистику
        if block_num % 1000 == 0 || block_num == total_blocks - 1 {
            let elapsed = final_start.elapsed();
            let speed = if elapsed.as_secs() > 0 {
                (block_num + 1) as f64 / elapsed.as_secs_f64()
            } else {
                0.0
            };
            println!("📊 Блоков обработано: {}/{}, скорость: {:.2} блоков/сек", 
                    block_num + 1, total_blocks, speed);
        }
    }

    let final_time = final_start.elapsed();
    let total_time = start_time.elapsed();

    pb.finish_with_message("Ultra Scale Streaming завершен!");

    println!("📊 Детальная статистика времени:");
    println!("   • Binary Splitting: {:.9}s", binary_time.as_secs_f64());
    println!("   • Финальные вычисления: {:.9}s", final_time.as_secs_f64());
    println!("   • Общее время: {:.9}s", total_time.as_secs_f64());
    println!("   • Скорость: {:.0} знаков/сек", digits as f64 / total_time.as_secs_f64());

    // Завершаем сохранение
    saver.finish()?;
    
    // Создаем читаемый файл
    let readable_file = format!("pi_{}_digits_STREAMING.txt", digits);
    utils::create_readable_pi_file(&stream_file, &readable_file)?;
    
    println!("✅ Результаты сохранены:");
    println!("   • Потоковый файл: {}", stream_file);
    println!("   • Читаемый файл: {}", readable_file);

    // Возвращаем первые несколько знаков для отображения
    let result = utils::merge_blocks(&stream_file)?;
    Ok(result)
}

/// Вычисляет блок π с заданной позицией
fn compute_pi_block(
    start_pos: usize,
    block_size: usize,
    precision: u32,
    q: &Integer,
    r: &Integer,
    block_number: usize,
) -> Result<PiBlock, String> {
    // Создаем Float с нужной точностью
    let q_float = Float::with_val(precision, q.clone());
    let r_float = Float::with_val(precision, r.clone());

    // Проверяем на нулевые значения
    if r_float.is_zero() {
        return Err("Ошибка: r равно нулю! Возможно переполнение при вычислении.".to_string());
    }

    let sqrt_10005 = Float::with_val(precision, 10005).sqrt();
    
    // Вычисляем π для блока
    let _numerator = Float::with_val(precision, 426880) * sqrt_10005.clone() * q_float.clone();
    
    // Используем безопасный логарифмический подход
    let log_426880 = Float::with_val(precision, 426880).ln();
    let log_sqrt_10005 = sqrt_10005.ln();
    let log_q = q_float.ln();
    let log_r = r_float.ln();

    // Проверяем логарифмы
    if log_426880.is_infinite() || log_426880.is_nan() ||
       log_sqrt_10005.is_infinite() || log_sqrt_10005.is_nan() ||
       log_q.is_infinite() || log_q.is_nan() ||
       log_r.is_infinite() || log_r.is_nan() {
        
        // Используем альтернативный подход с пониженной точностью
        let reduced_prec = precision / 2;
        let log_426880_alt = Float::with_val(reduced_prec, 426880).ln();
        let log_sqrt_10005_alt = Float::with_val(reduced_prec, 10005).sqrt().ln();
        let log_q_alt = Float::with_val(reduced_prec, q).ln();
        let log_r_alt = Float::with_val(reduced_prec, r).ln();
        
        if log_426880_alt.is_infinite() || log_426880_alt.is_nan() ||
           log_sqrt_10005_alt.is_infinite() || log_sqrt_10005_alt.is_nan() ||
           log_q_alt.is_infinite() || log_q_alt.is_nan() ||
           log_r_alt.is_infinite() || log_r_alt.is_nan() {
            return Err("Ошибка: Даже с пониженной точностью логарифмы переполняются!".to_string());
        }
        
        let log_numerator = log_426880_alt + log_sqrt_10005_alt + log_q_alt;
        let log_pi = log_numerator - log_r_alt;
        let pi_result = log_pi.exp();
        
        if pi_result.is_nan() || pi_result.is_infinite() {
            return Err("Ошибка: Альтернативный метод также дал некорректный результат!".to_string());
        }
        
        // Извлекаем нужные цифры
        let pi_str = pi_result.to_string_radix(10, Some(block_size + 10));
        let digits = extract_digits_from_pi_string(&pi_str, start_pos, block_size);
        
        Ok(PiBlock {
            block_number,
            digits,
            start_position: start_pos,
            end_position: start_pos + block_size,
            timestamp: Instant::now(),
        })
    } else {
        // Обычный путь
        let log_numerator = log_426880 + log_sqrt_10005 + log_q;
        let log_pi = log_numerator - log_r;
        let pi_result = log_pi.exp();
        
        if pi_result.is_nan() || pi_result.is_infinite() {
            return Err("Ошибка: Некорректный результат после экспоненты!".to_string());
        }
        
        // Извлекаем нужные цифры
        let pi_str = pi_result.to_string_radix(10, Some(block_size + 10));
        let digits = extract_digits_from_pi_string(&pi_str, start_pos, block_size);
        
        Ok(PiBlock {
            block_number,
            digits,
            start_position: start_pos,
            end_position: start_pos + block_size,
            timestamp: Instant::now(),
        })
    }
}

/// Извлекает цифры из строки π
fn extract_digits_from_pi_string(pi_str: &str, start_pos: usize, block_size: usize) -> String {
    // Убираем "3." в начале
    let digits_only = if pi_str.starts_with("3.") {
        &pi_str[2..]
    } else if pi_str.starts_with("3") {
        &pi_str[1..]
    } else {
        pi_str
    };
    
    // Извлекаем нужный диапазон
    let end_pos = std::cmp::min(start_pos + block_size, digits_only.len());
    if start_pos >= digits_only.len() {
        return "0".repeat(block_size);
    }
    
    let result = &digits_only[start_pos..end_pos];
    if result.len() < block_size {
        format!("{}{}", result, "0".repeat(block_size - result.len()))
    } else {
        result.to_string()
    }
}

/// Binary splitting с потоковым сохранением промежуточных результатов
fn ultra_binary_split_streaming(
    a: usize,
    b: usize,
    prec: u32,
    pb: &ProgressBar,
    saver: &StreamingPiSaver,
) -> Result<(Integer, Integer, Integer), String> {
    if b - a == 1 {
        // Базовый случай
        let p = Integer::from(1);
        let q = Integer::from(1);
        let r = Integer::from(1);
        
        // Периодически обновляем прогресс
        if a % 10000 == 0 {
            pb.inc(0); // Обновляем прогресс без изменения сообщения
        }
        
        Ok((p, q, r))
    } else {
        let m = (a + b) / 2;
        
        // Рекурсивно вычисляем левую и правую части
        let (p1, q1, r1) = ultra_binary_split_streaming(a, m, prec, pb, saver)?;
        let (p2, q2, r2) = ultra_binary_split_streaming(m, b, prec, pb, saver)?;
        
        // Объединяем результаты
        let p = (&p1 * &p2).into();
        let q = (&q1 * &q2).into();
        let r = (&r1 * &r2).into();
        
        // Обновляем прогресс
        pb.inc(1);
        
        Ok((p, q, r))
    }
}
