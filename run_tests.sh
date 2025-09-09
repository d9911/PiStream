#!/bin/bash

# Pi Calculator Test Suite
# Скрипт для запуска всех тестов и бенчмарков

echo "🧮 Pi Calculator - Comprehensive Test Suite"
echo "==========================================="
echo

# Проверяем, что мы в правильной директории
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Please run this script from the project root directory"
    exit 1
fi

# Функция для красивого вывода
print_section() {
    echo
    echo "🔹 $1"
    echo "$(printf '%.0s-' {1..50})"
}

# Функция для проверки успешности команды
check_result() {
    if [ $? -eq 0 ]; then
        echo "✅ $1 passed!"
    else
        echo "❌ $1 failed!"
        exit 1
    fi
}

# 1. Компиляция проекта
print_section "Компиляция проекта"
echo "Компилируем в release режиме для лучшей производительности..."
cargo build --release
check_result "Compilation"

# 2. Unit тесты
print_section "Unit Tests (в модуле pi.rs)"
echo "Запускаем встроенные unit тесты..."
cargo test --lib -- --nocapture
check_result "Unit tests"

# 3. Integration тесты
print_section "Integration Tests"
echo "Запускаем интеграционные тесты..."
cargo test --test integration_tests -- --nocapture
check_result "Integration tests"

# 4. Быстрый тест производительности
print_section "Quick Performance Test"
echo "Тестируем производительность всех алгоритмов на 100 знаках..."

echo "  🔸 Binary Splitting:"
time echo -e "2\n100" | cargo run --release --quiet 2>/dev/null | grep "Время выполнения"

echo "  🔸 Borwein Quadratic:"
time echo -e "4\n100" | cargo run --release --quiet 2>/dev/null | grep "Время выполнения"

echo "  🔸 Machin Formula:"
time echo -e "3\n100" | cargo run --release --quiet 2>/dev/null | grep "Время выполнения"

echo "  🔸 Naive Chudnovsky:"
time echo -e "1\n100" | cargo run --release --quiet 2>/dev/null | grep "Время выполнения"

# 5. Тест на больших числах (опционально)
print_section "Large Number Test (Optional)"
echo "Хотите протестировать вычисление 5000 знаков? Это может занять несколько секунд."
echo "Нажмите Enter для продолжения или Ctrl+C для пропуска..."
read -r

echo "Вычисляем 5000 знаков π с Binary Splitting..."
time echo -e "2\n5000" | cargo run --release --quiet 2>/dev/null | grep "Время выполнения"
check_result "Large number test"

# 6. Бенчмарки (если установлен criterion)
print_section "Benchmarks (если доступен)"
if command -v cargo &> /dev/null; then
    echo "Проверяем доступность бенчмарков..."
    if cargo bench --help &> /dev/null; then
        echo "Запускаем быстрые бенчмарки..."
        echo "Примечание: Полные бенчмарки могут занять несколько минут"
        echo "Запускаем только быстрые тесты на 50 знаках..."
        
        # Запускаем только быстрые бенчмарки
        cargo bench "Pi Algorithms - Small"
        check_result "Quick benchmarks"
    else
        echo "⚠️  Criterion benchmarks недоступны (это нормально)"
    fi
fi

# 7. Проверка корректности результатов
print_section "Accuracy Verification"
echo "Проверяем корректность вычислений..."

EXPECTED_PI_50="3.14159265358979323846264338327950288419716939937511"

echo "Проверяем каждый алгоритм на 50 знаках..."
for i in 1 2 3 4; do
    RESULT=$(echo -e "$i\n50" | cargo run --release --quiet 2>/dev/null | grep "π =" | cut -d' ' -f3)
    RESULT_50=${RESULT:0:52}
    
    if [ "$RESULT_50" = "$EXPECTED_PI_50" ]; then
        echo "  ✅ Алгоритм $i: корректный результат"
    else
        echo "  ❌ Алгоритм $i: неверный результат!"
        echo "     Ожидалось: $EXPECTED_PI_50"
        echo "     Получено:  $RESULT_50"
        exit 1
    fi
done

# 8. Финальная сводка
print_section "Test Summary"
echo "🎉 Все тесты прошли успешно!"
echo
echo "📊 Результаты:"
echo "  ✅ Компиляция: OK"
echo "  ✅ Unit тесты: OK"  
echo "  ✅ Integration тесты: OK"
echo "  ✅ Тест производительности: OK"
echo "  ✅ Тест на больших числах: OK"
echo "  ✅ Проверка корректности: OK"
echo
echo "🚀 Ваша программа для вычисления π работает отлично!"
echo "💡 Рекомендации:"
echo "  - Используйте Binary Splitting для >1000 знаков"
echo "  - Используйте Borwein для средних значений"
echo "  - Всегда используйте --release для максимальной производительности"
echo
echo "🔗 Для детальных бенчмарков запустите:"
echo "   cargo bench"
echo
