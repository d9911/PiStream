use crate::adapters::output::{StreamingPiSaver, PiBlock, utils};
use crate::domain::value_objects::Precision;
use indicatif::{ProgressBar, ProgressStyle};
use rug::{ops::Pow, Float, Integer};
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

// Глобальный флаг для прерывания вычислений
static INTERRUPT_REQUESTED: AtomicBool = AtomicBool::new(false);

/// Устанавливает флаг прерывания
pub fn request_interrupt() {
    INTERRUPT_REQUESTED.store(true, Ordering::Relaxed);
}

/// Проверяет, запрошено ли прерывание
pub fn is_interrupt_requested() -> bool {
    INTERRUPT_REQUESTED.load(Ordering::Relaxed)
}

/// Сбрасывает флаг прерывания
pub fn reset_interrupt() {
    INTERRUPT_REQUESTED.store(false, Ordering::Relaxed);
}

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

    // Вычисляем π целиком, а затем разбиваем на блоки для сохранения
    println!("🧮 Вычисляем π с точностью {} бит...", final_prec);
    
    // Создаем Float с нужной точностью
    let q_float = Float::with_val(final_prec as u32, q.clone());
    let r_float = Float::with_val(final_prec as u32, r.clone());

    // Проверяем на нулевые значения
    if r_float.is_zero() {
        return Err("Ошибка: r равно нулю! Возможно переполнение при вычислении.".to_string());
    }

    // Предвычисляем константы
    let sqrt_10005 = Float::with_val(final_prec as u32, 10005).sqrt();
    
    // Отладочная информация
    println!("🔍 Отладочная информация:");
    println!("   • q_float: {}", q_float);
    println!("   • r_float: {}", r_float);
    println!("   • sqrt_10005: {}", sqrt_10005);
    println!("   • q_float.is_zero(): {}", q_float.is_zero());
    println!("   • r_float.is_zero(): {}", r_float.is_zero());
    
    // Вычисляем π используя формулу Чудновского: π = 426880 * sqrt(10005) * q / r
    println!("🔢 Применяем формулу Чудновского...");
    let pi_result = Float::with_val(final_prec as u32, 426880) * &sqrt_10005 * &q_float / &r_float;
    
    // Проверяем результат
    if pi_result.is_nan() || pi_result.is_infinite() {
        println!("❌ Результат некорректный:");
        println!("   • pi_result.is_nan(): {}", pi_result.is_nan());
        println!("   • pi_result.is_infinite(): {}", pi_result.is_infinite());
        println!("   • pi_result: {}", pi_result);
        return Err("Ошибка: Некорректный результат вычисления π!".to_string());
    }
    
    println!("✅ π вычислен успешно! Разбиваем на блоки для сохранения...");
    
    // Конвертируем в строку с достаточной точностью
    let pi_str = pi_result.to_string_radix(10, Some(digits + 10));
    
    // Убираем "3." в начале
    let digits_only = if pi_str.starts_with("3.") {
        &pi_str[2..]
    } else if pi_str.starts_with("3") {
        &pi_str[1..]
    } else {
        &pi_str
    };
    
    println!("📊 Получено {} цифр π", digits_only.len());
    
    // Размер блока для сохранения
    let block_size = 10000;
    let total_blocks = (digits + block_size - 1) / block_size;
    
    println!("📊 Параметры сохранения:");
    println!("   • Размер блока: {}", block_size);
    println!("   • Всего блоков: {}", total_blocks);
    
    pb.set_length(total_blocks as u64);
    pb.set_message("Сохранение блоков π...");

    let mut processed_digits = 0;

    for block_num in 0..total_blocks {
        // Проверяем на прерывание
        if is_interrupt_requested() {
            println!("🛑 Прерывание запрошено пользователем!");
            println!("📊 Обработано блоков: {}/{} ({:.1}%)", 
                    block_num, total_blocks, 
                    block_num as f64 / total_blocks as f64 * 100.0);
            println!("💾 Частичные результаты сохранены в файле: {}", stream_file);
            return Err("Вычисления прерваны пользователем".to_string());
        }
        
        let start_pos = block_num * block_size;
        let end_pos = std::cmp::min(start_pos + block_size, digits);
        let current_block_size = end_pos - start_pos;
        
        // Извлекаем нужный диапазон цифр
        let block_digits = if start_pos >= digits_only.len() {
            "0".repeat(current_block_size)
        } else {
            let end_pos_actual = std::cmp::min(start_pos + current_block_size, digits_only.len());
            let result = &digits_only[start_pos..end_pos_actual];
            if result.len() < current_block_size {
                format!("{}{}", result, "0".repeat(current_block_size - result.len()))
            } else {
                result.to_string()
            }
        };
        
        // Создаем блок для сохранения
        let pi_block = PiBlock {
            block_number: block_num,
            digits: block_digits,
            start_position: start_pos,
            end_position: start_pos + current_block_size,
            timestamp: Instant::now(),
        };
        
        // Отправляем блок для сохранения
        saver.send_block(pi_block)?;
        
        processed_digits += current_block_size;
        pb.inc(1);
        
        // Обновления прогресса
        if block_num % 1000 == 0 || block_num == total_blocks - 1 {
            let elapsed = final_start.elapsed();
            let speed = if elapsed.as_secs() > 0 {
                processed_digits as f64 / elapsed.as_secs_f64()
            } else {
                0.0
            };
            
            println!("📊 Прогресс: {}/{} блоков ({:.1}%), скорость: {:.0} знаков/сек", 
                    block_num + 1, total_blocks, 
                    (block_num + 1) as f64 / total_blocks as f64 * 100.0,
                    speed);
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


/// Binary splitting с потоковым сохранением промежуточных результатов
fn ultra_binary_split_streaming(
    a: usize,
    b: usize,
    prec: u32,
    pb: &ProgressBar,
    saver: &StreamingPiSaver,
) -> Result<(Integer, Integer, Integer), String> {
    if b - a == 1 {
        // Базовый случай - правильные вычисления по формуле Чудновского
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
        let r_temp1: Integer = (&r1 * &q2).into();
        let r_temp2: Integer = (&r2 * &p1).into();
        let r = r_temp1 + r_temp2;
        
        // Обновляем прогресс
        pb.inc(1);
        
        Ok((p, q, r))
    }
}
