mod pi;

use std::io::{self, Write};
use pi::PiAlgorithm;

fn main() {
    println!("🔢 Высокоточное вычисление числа π");
    println!("=====================================");
    
    // Выбор алгоритма
    println!("\nДоступные алгоритмы:");
    println!("1. Чудновский (наивный) - стандартная реализация (макс: 1M знаков)");
    println!("2. Чудновский (Binary Splitting) - самый быстрый для больших чисел (макс: 10M знаков)");
    println!("3. Формула Мачина - классический быстрый метод (макс: 5M знаков)");
    println!("4. Алгоритм Борвейна - квадратичная сходимость (макс: 2M знаков)");
    
    print!("Выберите алгоритм [1-4, по умолчанию: 2]: ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let algorithm_choice = input.trim();
    
    let algorithm = match algorithm_choice {
        "1" => PiAlgorithm::ChudnovskyNaive,
        "3" => PiAlgorithm::MachinFormula,
        "4" => PiAlgorithm::BorweinQuadratic,
        _ => {
            if !algorithm_choice.is_empty() && algorithm_choice != "2" {
                println!("Неверный выбор. Используется Binary Splitting (самый быстрый).");
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
            Ok(n) if n > 0 && n <= 50_000_000 => n,
            Ok(n) if n > 50_000_000 => {
                println!("⚠️  Слишком большое значение! Максимум: 50,000,000 знаков.");
                println!("Используется значение по умолчанию: 1000");
                1000
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
    };
    
    println!("\n🚀 Вычисление π с {} знаками после запятой...", digits);
    println!("📊 Алгоритм: {}", algorithm_name);
    println!("⏱️  Начинаем вычисление...\n");

    // Проверяем совместимость алгоритма с количеством знаков
    let max_digits_for_algorithm = match algorithm {
        PiAlgorithm::ChudnovskyNaive => 1_000_000,
        PiAlgorithm::ChudnovskyBinarySplitting => 10_000_000,
        PiAlgorithm::MachinFormula => 5_000_000,
        PiAlgorithm::BorweinQuadratic => 2_000_000,
    };
    
    if digits > max_digits_for_algorithm {
        println!("❌ Ошибка: Выбранный алгоритм не поддерживает {} знаков!", digits);
        println!("📊 Максимум для {}: {} знаков", algorithm_name, max_digits_for_algorithm);
        println!("💡 Рекомендация: Используйте Binary Splitting для больших вычислений");
        return;
    }

    let start_time = std::time::Instant::now();
    
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
    
    let elapsed = start_time.elapsed();

    println!("\n✅ Вычисление завершено!");
    println!("⏱️  Время выполнения: {:.2?}", elapsed);
    
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
