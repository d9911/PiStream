use crate::domain::entities::{Algorithm, PiComputation};
use crate::domain::services::ComputationEstimate;
use crate::ports::input::AlgorithmInfo;
use crate::ports::output::UiPresenter;
use std::io::{self, Write};

/// Console-based UI presenter for displaying computation progress and results
pub struct ConsolePresenter;

impl ConsolePresenter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ConsolePresenter {
    fn default() -> Self {
        Self::new()
    }
}

impl UiPresenter for ConsolePresenter {
    fn show_welcome(&self) {
        println!("🚀 Добро пожаловать в PiStreamTerminal!");
        println!("Вычислим π с высокой точностью!");
        println!();
    }

    fn show_algorithms(&self, algorithms: &[AlgorithmInfo]) {
        println!("Доступные алгоритмы:");
        for info in algorithms {
            println!("  {}", info.format_for_menu());
        }
        println!();
    }

    fn prompt_algorithm_selection(&self) -> String {
        print!(
            "Выберите алгоритм (1-{}): ",
            Algorithm::ChudnovskyUltraScale as u8
        );
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Ошибка чтения ввода. Используем значение по умолчанию: 2");
            input = "2".to_string();
        }
        input.trim().to_string()
    }

    fn prompt_digits_input(&self) -> String {
        print!("Введите количество знаков π (Enter для значения по умолчанию): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Ошибка чтения ввода. Используем значение по умолчанию: 1000");
            input = "1000".to_string();
        }
        input.trim().to_string()
    }

    fn show_computation_start(&self, algorithm: Algorithm, digits: usize) {
        println!("🎯 Начинаем вычисление π...");
        println!("Алгоритм: {}", algorithm.display_name());
        println!("Знаков: {}", digits);
        println!();
    }

    fn show_resource_estimation(&self, estimate: &ComputationEstimate) {
        println!("📊 Оценка ресурсов:");
        println!("  Время: {}", estimate.estimated_time);
        println!("  Память: {:.2} GB", estimate.memory_gb);
        println!("  Итераций: {}", estimate.iterations);
        println!();
    }

    fn show_results(&self, computation: &PiComputation) {
        println!("✅ Результат:");
        if let Some(result) = computation.display_result(Some(100)) {
            println!("π = {}", result);
        } else {
            println!("π = [вычисление не завершено]");
        }
        println!();
    }

    fn show_statistics(&self, computation: &PiComputation) {
        println!("📈 Статистика:");
        println!("  Алгоритм: {}", computation.algorithm.display_name());
        println!("  Знаков: {}", computation.digits.value());
        if let Some(total_time) = computation.total_time {
            println!("  Время: {:.2}s", total_time.as_secs_f64());
            if let Some(speed) = computation.speed_digits_per_second() {
                println!("  Скорость: {:.0} знаков/сек", speed);
            }
        }
        println!();
    }

    fn show_achievement(&self, digits: usize) {
        let achievement = match digits {
            0..=1000 => "Новичок",
            1001..=10000 => "Любитель",
            10001..=100000 => "Энтузиаст",
            100001..=1000000 => "Мастер",
            1000001..=10000000 => "Эксперт",
            10000001..=100000000 => "Легенда",
            _ => "🚀 ULTRA SCALE MASTER",
        };

        println!("🏆 Достижение: {} ({} знаков)", achievement, digits);
        println!();
    }

    fn prompt_save_to_file(&self) -> bool {
        print!("Сохранить результат в файл? (y/n): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Ошибка чтения ввода. Используем значение по умолчанию: n");
            input = "n".to_string();
        }
        input.trim().to_lowercase() == "y" || input.trim().to_lowercase() == "yes"
    }

    fn show_save_result(&self, filename: &str, success: bool, error: Option<&str>) {
        if success {
            println!("✅ Файл сохранен: {}", filename);
        } else {
            println!(
                "❌ Ошибка сохранения: {}",
                error.unwrap_or("неизвестная ошибка")
            );
        }
    }

    fn show_error(&self, message: &str) {
        eprintln!("❌ Ошибка: {}", message);
    }

    fn show_warning(&self, message: &str) {
        println!("⚠️  Предупреждение: {}", message);
    }

    fn show_info(&self, message: &str) {
        println!("ℹ️  Информация: {}", message);
    }

    fn show_ultra_scale_warning(&self) {
        println!("🚨 ULTRA SCALE ПРЕДУПРЕЖДЕНИЕ 🚨");
        println!("Вы выбрали алгоритм для триллионов знаков!");
        println!("Это может занять очень много времени и памяти.");
        println!("Убедитесь, что у вас достаточно ресурсов.");
        println!();
    }

    fn prompt_ultra_scale_confirmation(&self, digits: usize) -> bool {
        println!("Вы действительно хотите вычислить {} знаков π?", digits);
        print!("Это может занять много времени! Продолжить? (y/n): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Ошибка чтения ввода. Используем значение по умолчанию: n");
            input = "n".to_string();
        }
        input.trim().to_lowercase() == "y" || input.trim().to_lowercase() == "yes"
    }
}
