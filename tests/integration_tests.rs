use pi_stream_terminal::{compute_pi, PiAlgorithm};
use std::time::Instant;

/// Эталонное значение π с 100 знаками после запятой
const PI_100_DIGITS: &str = "3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170680";

/// Эталонное значение π с 50 знаками после запятой
const PI_50_DIGITS: &str = "3.14159265358979323846264338327950288419716939937511";

/// Эталонное значение π с 10 знаками после запятой
const PI_10_DIGITS: &str = "3.1415926536";

#[test]
fn test_all_algorithms_correctness_10_digits() {
    println!("🧪 Тестирование корректности всех алгоритмов (10 знаков)");

    let algorithms = [
        (PiAlgorithm::ChudnovskyNaive, "Чудновский (наивный)"),
        (
            PiAlgorithm::ChudnovskyBinarySplitting,
            "Чудновский (Binary Splitting)",
        ),
        (PiAlgorithm::MachinFormula, "Формула Мачина"),
        (PiAlgorithm::BorweinQuadratic, "Алгоритм Борвейна"),
    ];

    for (algorithm, name) in algorithms {
        println!("  ✓ Тестируем {}", name);
        let result = compute_pi(10, algorithm);
        let result_truncated = &result[..12]; // "3." + 10 знаков
        assert_eq!(
            result_truncated, PI_10_DIGITS,
            "Алгоритм {} дал неверный результат",
            name
        );
    }

    println!("✅ Все алгоритмы прошли тест на 10 знаков!");
}

#[test]
fn test_all_algorithms_correctness_50_digits() {
    println!("🧪 Тестирование корректности всех алгоритмов (50 знаков)");

    let algorithms = [
        (PiAlgorithm::ChudnovskyNaive, "Чудновский (наивный)"),
        (
            PiAlgorithm::ChudnovskyBinarySplitting,
            "Чудновский (Binary Splitting)",
        ),
        (PiAlgorithm::MachinFormula, "Формула Мачина"),
        (PiAlgorithm::BorweinQuadratic, "Алгоритм Борвейна"),
    ];

    for (algorithm, name) in algorithms {
        println!("  ✓ Тестируем {}", name);
        let result = compute_pi(50, algorithm);
        let result_truncated = &result[..52]; // "3." + 50 знаков
        assert_eq!(
            result_truncated, PI_50_DIGITS,
            "Алгоритм {} дал неверный результат",
            name
        );
    }

    println!("✅ Все алгоритмы прошли тест на 50 знаков!");
}

#[test]
fn test_chudnovsky_binary_splitting_100_digits() {
    println!("🧪 Тестирование Binary Splitting на 100 знаках");

    let result = compute_pi(100, PiAlgorithm::ChudnovskyBinarySplitting);
    let result_truncated = &result[..102]; // "3." + 100 знаков
    assert_eq!(
        result_truncated, PI_100_DIGITS,
        "Binary Splitting дал неверный результат на 100 знаках"
    );

    println!("✅ Binary Splitting прошел тест на 100 знаков!");
}

#[test]
fn test_performance_comparison_small() {
    println!("⏱️  Тестирование производительности на 50 знаках");

    let algorithms = [
        (PiAlgorithm::ChudnovskyNaive, "Чудновский (наивный)"),
        (
            PiAlgorithm::ChudnovskyBinarySplitting,
            "Чудновский (Binary Splitting)",
        ),
        (PiAlgorithm::MachinFormula, "Формула Мачина"),
        (PiAlgorithm::BorweinQuadratic, "Алгоритм Борвейна"),
    ];

    let mut results = Vec::new();

    for (algorithm, name) in algorithms {
        let start = Instant::now();
        let _result = compute_pi(50, algorithm);
        let duration = start.elapsed();

        results.push((name, duration));
        println!("  {} : {:?}", name, duration);
    }

    // Проверяем, что все алгоритмы работают достаточно быстро (< 10ms для 50 знаков)
    for (name, duration) in &results {
        assert!(
            duration.as_millis() < 10,
            "Алгоритм {} слишком медленный: {:?}",
            name,
            duration
        );
    }

    println!("✅ Все алгоритмы показали хорошую производительность!");
}

#[test]
fn test_performance_binary_splitting_1000_digits() {
    println!("⚡ Тестирование производительности Binary Splitting на 1000 знаках");

    let start = Instant::now();
    let result = compute_pi(1000, PiAlgorithm::ChudnovskyBinarySplitting);
    let duration = start.elapsed();

    println!("  Binary Splitting (1000 знаков): {:?}", duration);

    // Проверяем, что результат не пустой и начинается с "3.14"
    assert!(
        result.starts_with("3.14"),
        "Результат должен начинаться с 3.14"
    );
    assert!(
        result.len() >= 1002,
        "Результат должен содержать минимум 1002 символа"
    ); // "3." + 1000 знаков

    // Проверяем, что алгоритм работает достаточно быстро (< 100ms для 1000 знаков)
    assert!(
        duration.as_millis() < 100,
        "Binary Splitting слишком медленный для 1000 знаков: {:?}",
        duration
    );

    println!("✅ Binary Splitting показал отличную производительность!");
}

#[test]
fn test_edge_cases() {
    println!("🎯 Тестирование граничных случаев");

    // Тест на минимальное количество знаков
    println!("  ✓ Тестируем 1 знак");
    let result = compute_pi(1, PiAlgorithm::ChudnovskyBinarySplitting);
    assert!(
        result.starts_with("3.1"),
        "Результат для 1 знака должен быть 3.1"
    );

    // Тест на очень маленькое количество знаков
    println!("  ✓ Тестируем 5 знаков");
    let result = compute_pi(5, PiAlgorithm::BorweinQuadratic);
    assert!(
        result.starts_with("3.14159"),
        "Результат для 5 знаков неверный"
    );

    println!("✅ Граничные случаи прошли тесты!");
}

#[test]
fn test_algorithm_consistency() {
    println!("🔄 Тестирование консистентности алгоритмов");

    // Проверяем, что все алгоритмы дают одинаковый результат на 20 знаках
    let digits = 20;
    let algorithms = [
        PiAlgorithm::ChudnovskyNaive,
        PiAlgorithm::ChudnovskyBinarySplitting,
        PiAlgorithm::MachinFormula,
        PiAlgorithm::BorweinQuadratic,
    ];

    let mut results = Vec::new();
    for algorithm in algorithms {
        let result = compute_pi(digits, algorithm);
        let truncated = &result[..22]; // "3." + 20 знаков
        results.push(truncated.to_string());
    }

    // Все результаты должны быть одинаковыми
    let first_result = &results[0];
    for (i, result) in results.iter().enumerate() {
        assert_eq!(
            result, first_result,
            "Алгоритм {} дал отличающийся результат",
            i
        );
    }

    println!("✅ Все алгоритмы дают консистентные результаты!");
    println!("  Результат: {}", first_result);
}

#[test]
#[ignore] // Игнорируем по умолчанию, так как тест медленный
fn test_large_computation() {
    println!("🚀 Тестирование больших вычислений (5000 знаков)");

    let start = Instant::now();
    let result = compute_pi(5000, PiAlgorithm::ChudnovskyBinarySplitting);
    let duration = start.elapsed();

    println!("  Binary Splitting (5000 знаков): {:?}", duration);

    // Проверяем корректность результата
    assert!(
        result.starts_with("3.14159265358979323846264338327950288419716939937510"),
        "Результат для 5000 знаков начинается неверно"
    );
    assert!(
        result.len() >= 5002,
        "Результат должен содержать минимум 5002 символа"
    );

    // Проверяем разумное время выполнения (< 10 секунд)
    assert!(
        duration.as_secs() < 10,
        "Вычисление 5000 знаков заняло слишком много времени: {:?}",
        duration
    );

    println!("✅ Большое вычисление прошло успешно!");
}

/// Вспомогательная функция для запуска всех быстрых тестов
#[test]
fn run_all_fast_tests() {
    println!("🏃 Запуск всех быстрых тестов...\n");

    test_all_algorithms_correctness_10_digits();
    println!();

    test_all_algorithms_correctness_50_digits();
    println!();

    test_chudnovsky_binary_splitting_100_digits();
    println!();

    test_performance_comparison_small();
    println!();

    test_performance_binary_splitting_1000_digits();
    println!();

    test_edge_cases();
    println!();

    test_algorithm_consistency();

    println!("\n🎉 Все быстрые тесты прошли успешно!");
}
