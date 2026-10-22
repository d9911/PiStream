#!/bin/bash

# Скрипт для запуска PiStreamTerminal с мониторингом
# Использование: ./run_with_monitoring.sh

echo "🚀 Запуск PiStreamTerminal с улучшенным мониторингом"
echo "=================================================="

# Проверяем, что мы в правильной директории
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Ошибка: Запустите скрипт из корневой директории проекта"
    exit 1
fi

# Компилируем проект
echo "🔨 Компилируем проект..."
if ! cargo build --release --locked --bin pistream-cli; then
    echo "❌ Ошибка компиляции!"
    exit 1
fi

echo "✅ Компиляция завершена успешно!"
echo ""

# Создаем функцию для мониторинга
monitor_process() {
    local pid=$1
    local start_time=$(date +%s)
    
    echo "📊 Мониторинг процесса PID: $pid"
    echo "⏰ Время запуска: $(date)"
    echo ""
    
    while kill -0 $pid 2>/dev/null; do
        # Получаем статистику процесса
        local stats=$(ps -p $pid -o pid,ppid,pcpu,pmem,etime,command --no-headers 2>/dev/null)
        
        if [ -n "$stats" ]; then
            # Показываем статистику каждые 10 секунд
            local current_time=$(date +%s)
            local elapsed=$((current_time - start_time))
            
            if [ $((elapsed % 10)) -eq 0 ]; then
                echo "📈 [$elapsed сек] $stats"
                
                # Показываем размер файлов результатов
                local files=$(ls -lh pi_stream_*_digits.txt 2>/dev/null | wc -l)
                if [ $files -gt 0 ]; then
                    echo "💾 Файлы результатов:"
                    ls -lh pi_stream_*_digits.txt 2>/dev/null | tail -3
                fi
                
                # Показываем последние строки прогресса
                if [ -f "pi_stream_200000000_digits.txt" ]; then
                    local last_line=$(tail -1 pi_stream_200000000_digits.txt 2>/dev/null | grep -v "^#")
                    if [ -n "$last_line" ]; then
                        echo "📝 Последний блок: $last_line"
                    fi
                fi
                echo ""
            fi
        fi
        
        sleep 1
    done
    
    echo "✅ Процесс завершен!"
}

# Запускаем приложение в фоне
echo "🎯 Запускаем PiStreamTerminal..."
./target/release/pistream-cli &
APP_PID=$!

# Даем время на запуск
sleep 2

# Проверяем, что процесс запустился
if ! kill -0 $APP_PID 2>/dev/null; then
    echo "❌ Процесс не запустился!"
    exit 1
fi

echo "✅ Процесс запущен с PID: $APP_PID"
echo ""

# Запускаем мониторинг
monitor_process $APP_PID

# Ждем завершения процесса
wait $APP_PID
EXIT_CODE=$?

echo ""
echo "🏁 Приложение завершено с кодом: $EXIT_CODE"

# Показываем финальную статистику
echo ""
echo "📊 Финальная статистика:"
if [ -f "pi_stream_200000000_digits.txt" ]; then
    local file_size=$(ls -lh pi_stream_200000000_digits.txt | awk '{print $5}')
    local line_count=$(wc -l < pi_stream_200000000_digits.txt)
    echo "📁 Размер файла: $file_size"
    echo "📄 Количество строк: $line_count"
    
    # Показываем первые несколько цифр π
    local first_digits=$(head -5 pi_stream_200000000_digits.txt | grep -v "^#" | head -1 | cut -d: -f4 | head -c 50)
    if [ -n "$first_digits" ]; then
        echo "🔢 Первые цифры π: 3.$first_digits..."
    fi
fi

echo ""
echo "🎉 Мониторинг завершен!"
