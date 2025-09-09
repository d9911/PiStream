mod pi;

use std::io::{self, Write};
use pi::PiAlgorithm;

fn main() {
    println!("🔢 Высокоточное вычисление числа π");
    println!("=====================================");
    
    // Выбор алгоритма
    println!("\nДоступные алгоритмы:");
    println!("1. Чудновский (наивный) - стандартная реализация (макс: 1M знаков)");
    println!("2. Чудновский (Binary Splitting) - быстрый для больших чисел (макс: 100M знаков)");
    println!("3. Формула Мачина - классический быстрый метод (макс: 5M знаков)");
    println!("4. Алгоритм Борвейна - квадратичная сходимость (макс: 2M знаков)");
    println!("5. 🚀 ULTRA SCALE Чудновский - для триллионов знаков (макс: ∞)");
    
    print!("Выберите алгоритм [1-5, по умолчанию: 2]: ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let algorithm_choice = input.trim();
    
    let algorithm = match algorithm_choice {
        "1" => PiAlgorithm::ChudnovskyNaive,
        "3" => PiAlgorithm::MachinFormula,
        "4" => PiAlgorithm::BorweinQuadratic,
        "5" => {
            println!("⚠️  ULTRA SCALE режим выбран!");
            println!("🚀 Этот алгоритм может вычислять триллионы знаков π");
            PiAlgorithm::ChudnovskyUltraScale
        },
        _ => {
            if !algorithm_choice.is_empty() && algorithm_choice != "2" {
                println!("Неверный выбор. Используется Binary Splitting (быстрый).");
            }
            PiAlgorithm::ChudnovskyBinarySplitting
        }
    };
    
    // Выбор количества знаков
    print!("Введите количество знаков после запятой [по умолчанию: 1000]: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    let digits: usize = if input.is_empty() {
        1000
    } else {
        match input.parse() {
            Ok(n) if n > 0 => {
                // Для Ultra Scale алгоритма нет лимитов
                if matches!(algorithm, PiAlgorithm::ChudnovskyUltraScale) {
                    n
                } else if n <= 1_000_000_000 {
                    n
                } else {
                    println!("⚠️  Слишком большое значение для выбранного алгоритма!");
                    println!("💡 Используйте ULTRA SCALE (алгоритм 5) для триллионов знаков");
                    println!("Используется значение по умолчанию: 1000");
                    1000
                }
            }
            _ => {
                println!("Неверный ввод. Используется значение по умолчанию: 1000");
                1000
            }
        }
    };

    let algorithm_name = match algorithm {
        PiAlgorithm::ChudnovskyNaive => "Чудновский (наивный)",
        PiAlgorithm::ChudnovskyBinarySplitting => "Чудновский (Binary Splitting)",
        PiAlgorithm::MachinFormula => "Формула Мачина",
        PiAlgorithm::BorweinQuadratic => "Алгоритм Борвейна",
        PiAlgorithm::ChudnovskyUltraScale => "🚀 ULTRA SCALE Чудновский",
    };
    
    println!("\n🚀 Вычисление π с {} знаками после запятой...", digits);
    println!("📊 Алгоритм: {}", algorithm_name);
    println!("⏱️  Начинаем вычисление...\n");

    // Проверяем совместимость алгоритма с количеством знаков
    let max_digits_for_algorithm = match algorithm {
        PiAlgorithm::ChudnovskyNaive => 1_000_000,
        PiAlgorithm::ChudnovskyBinarySplitting => 100_000_000,
        PiAlgorithm::MachinFormula => 5_000_000,
        PiAlgorithm::BorweinQuadratic => 2_000_000,
        PiAlgorithm::ChudnovskyUltraScale => usize::MAX, // Без лимитов!
    };
    
    if digits > max_digits_for_algorithm {
        println!("❌ Ошибка: Выбранный алгоритм не поддерживает {} знаков!", digits);
        println!("📊 Максимум для {}: {} знаков", algorithm_name, max_digits_for_algorithm);
        println!("💡 Рекомендация: Используйте Binary Splitting для больших вычислений");
        return;
    }

    println!("⏰ Засекаем время...");
    let total_start = std::time::Instant::now();
    let computation_start = std::time::Instant::now();
    
    let pi_str = match std::panic::catch_unwind(|| {
        pi::compute_pi(digits, algorithm)
    }) {
        Ok(result) => result,
        Err(_) => {
            println!("❌ Произошла ошибка при вычислении!");
            println!("💡 Попробуйте уменьшить количество знаков или выберите другой алгоритм");
            return;
        }
    };
    
    let computation_time = computation_start.elapsed();
    
    println!("🔄 Подготовка результата...");
    let display_start = std::time::Instant::now();

    // Показываем результат с разбивкой для удобства чтения
    if digits <= 100 {
        println!("π = {}", pi_str);
    } else {
        // Показываем первые 50 знаков, затем многоточие, затем последние 10
        let pi_chars: Vec<char> = pi_str.chars().collect();
        if pi_chars.len() > 60 {
            let first_part: String = pi_chars[..52].iter().collect(); // включая "3."
            let last_part: String = pi_chars[pi_chars.len()-10..].iter().collect();
            println!("π = {}...{}", first_part, last_part);
            println!("📝 Полный результат содержит {} цифр после запятой", digits);
        } else {
            println!("π = {}", pi_str);
        }
    }
    
    let display_time = display_start.elapsed();
    let total_time = total_start.elapsed();
    
    println!("\n✅ Вычисление завершено!");
    
    // Детальная статистика времени
    println!("\n📊 Детальная статистика времени:");
    println!("   • Основные вычисления: {:?}", computation_time);
    println!("   • Подготовка результата: {:?}", display_time);
    println!("   • Общее время: {:?}", total_time);
    
    // Дополнительная статистика
    if total_time.as_secs() > 0 {
        println!("   • Скорость: {:.0} знаков/сек", digits as f64 / total_time.as_secs_f64());
    } else {
        println!("   • Скорость: {:.0} знаков/мс", digits as f64 / total_time.as_millis() as f64);
    }
    
    // Показываем эффективность в зависимости от размера
    if digits >= 1_000_000 {
        println!("   • Эффективность: {:.2} мкс/знак", total_time.as_micros() as f64 / digits as f64);
    }
    
    // Сравнение с мировыми рекордами
    if digits >= 1_000_000_000_000 {
        println!("\n🏆 ПОЗДРАВЛЯЕМ! Вы вычислили триллион знаков π!");
        println!("   📈 Для сравнения: мировой рекорд - 300 триллионов знаков (7.5 месяцев)");
        println!("   🎯 Ваш результат: {} знаков за {:?}", digits, total_time);
    } else if digits >= 1_000_000_000 {
        println!("\n🎉 Отлично! Вы вычислили миллиард знаков π!");
    } else if digits >= 1_000_000 {
        println!("\n🎊 Здорово! Вы вычислили миллион знаков π!");
    }
    
    // Предложение сохранить результат
    if digits >= 1000 {
        print!("\n💾 Сохранить полный результат в файл? [y/N]: ");
        io::stdout().flush().unwrap();
        
        let mut save_input = String::new();
        io::stdin().read_line(&mut save_input).unwrap();
        
        if save_input.trim().to_lowercase() == "y" || save_input.trim().to_lowercase() == "yes" {
            let filename = format!("pi_{}_digits_{}.txt", digits, algorithm_name.replace(" ", "_").replace("(", "").replace(")", ""));
            match std::fs::write(&filename, &pi_str) {
                Ok(_) => println!("✅ Результат сохранен в файл: {}", filename),
                Err(e) => println!("❌ Ошибка при сохранении: {}", e),
            }
        }
    }
}
