#!/bin/bash

# Быстрый запуск PiStreamTerminal
# Использование: ./quick_run.sh

echo "⚡ Быстрый запуск PiStreamTerminal"
echo "================================="

# Проверяем, что мы в правильной директории
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Ошибка: Запустите скрипт из корневой директории проекта"
    exit 1
fi

# Компилируем в debug режиме для быстрой компиляции
echo "🔨 Быстрая компиляция (debug режим)..."
if ! cargo build; then
    echo "❌ Ошибка компиляции!"
    exit 1
fi

echo "✅ Компиляция завершена!"
echo ""

# Запускаем приложение
echo "🚀 Запускаем PiStreamTerminal..."
echo "💡 Для прерывания нажмите Ctrl+C"
echo ""

./target/debug/PiStreamTerminal
