#!/bin/bash

# Тест правильности вычисления π
echo "🧪 Тестируем правильность вычисления π"
echo "====================================="

# Удаляем старые файлы
echo "🧹 Очищаем старые файлы..."
rm -f pi_stream_1000_digits.txt pi_1000_digits_STREAMING.txt pi_1000_digits_*.txt

# Компилируем
echo "🔨 Компилируем..."
cargo build

# Запускаем тест с 1000 знаков
echo "🚀 Запускаем тест с 1000 знаков..."
echo "6" | echo "1000" | echo "y" | ./target/debug/PiStreamTerminal

echo ""
echo "📊 Проверяем результат..."

# Проверяем файлы
if [ -f "pi_stream_1000_digits.txt" ]; then
    echo "✅ Потоковый файл создан: pi_stream_1000_digits.txt"
    
    # Показываем первые несколько строк
    echo "📝 Первые строки потокового файла:"
    head -3 pi_stream_1000_digits.txt
    
    # Проверяем формат блоков
    echo ""
    echo "🔍 Проверка формата блоков:"
    grep -v "^#" pi_stream_1000_digits.txt | head -2
else
    echo "❌ Потоковый файл не создан!"
fi

if [ -f "pi_1000_digits_STREAMING.txt" ]; then
    echo ""
    echo "✅ Читаемый файл создан: pi_1000_digits_STREAMING.txt"
    
    # Показываем первые цифры π
    echo "🔢 Первые цифры π:"
    head -1 pi_1000_digits_STREAMING.txt | head -c 50
    echo "..."
    
    # Проверяем, что начинается с 3.14
    first_line=$(head -1 pi_1000_digits_STREAMING.txt)
    if [[ $first_line == π\ =\ 3.14* ]]; then
        echo "✅ Правильно! Начинается с 3.14..."
    else
        echo "❌ Ошибка! Не начинается с 3.14..."
        echo "Получено: $first_line"
    fi
else
    echo "❌ Читаемый файл не создан!"
fi

# Проверяем количество файлов
file_count=$(ls pi_1000_digits_*.txt 2>/dev/null | wc -l)
echo ""
echo "📁 Количество созданных файлов: $file_count"

if [ $file_count -eq 2 ]; then
    echo "✅ Правильно! Создано только 2 файла"
else
    echo "❌ Ошибка! Создано $file_count файлов вместо 2"
    echo "Файлы:"
    ls pi_1000_digits_*.txt 2>/dev/null
fi
