#!/bin/bash

# Скрипт для мониторинга вычисления π
# Использование: ./monitor_pi.sh

echo "🔍 Мониторинг вычисления π"
echo "=========================="

# Функция для показа статистики
show_stats() {
    echo ""
    echo "📊 Статистика процесса:"
    
    # Находим процесс PiStreamTerminal
    PID=$(pgrep -f "PiStreamTerminal")
    if [ -z "$PID" ]; then
        echo "❌ Процесс PiStreamTerminal не найден"
        return 1
    fi
    
    echo "🆔 PID процесса: $PID"
    
    # Показываем использование CPU и памяти
    ps -p $PID -o pid,ppid,pcpu,pmem,etime,command --no-headers | while read line; do
        echo "📈 $line"
    done
    
    # Показываем размер файлов результатов
    echo ""
    echo "💾 Файлы результатов:"
    ls -lh pi_stream_*_digits.txt 2>/dev/null | while read line; do
        echo "📁 $line"
    done
    
    # Показываем последние строки лога (если есть)
    if [ -f "pi_stream_200000000_digits.txt" ]; then
        echo ""
        echo "📝 Последние сохраненные блоки:"
        tail -5 pi_stream_200000000_digits.txt | grep -v "^#" | head -3
    fi
}

# Функция для прерывания процесса
interrupt_process() {
    PID=$(pgrep -f "PiStreamTerminal")
    if [ -n "$PID" ]; then
        echo "🛑 Отправляем сигнал прерывания процессу $PID..."
        kill -INT $PID
        echo "✅ Сигнал отправлен. Процесс должен завершиться корректно."
    else
        echo "❌ Процесс не найден"
    fi
}

# Основной цикл мониторинга
echo "💡 Команды:"
echo "   s - показать статистику"
echo "   i - прервать процесс"
echo "   q - выйти"
echo "   Enter - обновить статистику"
echo ""

while true; do
    read -p "Введите команду: " cmd
    
    case $cmd in
        s|S)
            show_stats
            ;;
        i|I)
            interrupt_process
            ;;
        q|Q)
            echo "👋 До свидания!"
            exit 0
            ;;
        "")
            show_stats
            ;;
        *)
            echo "❓ Неизвестная команда. Используйте s, i, q или Enter"
            ;;
    esac
done
