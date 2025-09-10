# Hexagonal Architecture (Ports and Adapters)

## 🏗️ Архитектурный обзор

Проект был реорганизован с использованием **Hexagonal Architecture** (также известной как **Ports and Adapters**) для улучшения модульности, тестируемости и расширяемости кода.

### 🎯 Принципы архитектуры

1. **Разделение ответственности**: Четкое разделение бизнес-логики от внешних зависимостей
2. **Инверсия зависимостей**: Зависимости направлены от внешних слоев к внутренним
3. **Тестируемость**: Каждый слой может тестироваться независимо
4. **Расширяемость**: Легко добавлять новые адаптеры без изменения ядра

## 📁 Структура проекта

```
src/
├── main.rs                    # Entry point
├── lib.rs                     # Library root with facade
├── domain/                    # 🎯 CORE - Бизнес-логика
│   ├── entities/              # Сущности предметной области
│   │   ├── algorithm.rs       # Алгоритмы вычисления π
│   │   └── pi_computation.rs  # Вычисление π как сущность
│   ├── value_objects/         # Объекты-значения
│   │   ├── digits.rs          # Количество знаков
│   │   └── precision.rs       # Точность вычислений
│   └── services/              # Доменные сервисы
│       └── pi_calculator.rs   # Основная логика вычислений
├── ports/                     # 🔌 ПОРТЫ - Интерфейсы
│   ├── input/                 # Входящие порты (use cases)
│   │   ├── pi_computation_use_case.rs
│   │   └── progress_reporter.rs
│   └── output/                # Исходящие порты (репозитории)
│       ├── pi_repository.rs
│       ├── file_writer.rs
│       └── ui_presenter.rs
├── adapters/                  # 🔧 АДАПТЕРЫ - Реализации
│   ├── input/                 # Входящие адаптеры
│   │   ├── cli_handler.rs
│   │   └── console_progress.rs
│   ├── output/                # Исходящие адаптеры
│   │   ├── file_storage.rs
│   │   └── console_presenter.rs
│   └── algorithms/            # Алгоритмы вычисления
│       ├── algorithm_repository.rs
│       ├── chudnovsky_naive.rs
│       ├── chudnovsky_binary_splitting.rs
│       ├── chudnovsky_ultra_scale.rs
│       ├── machin_formula.rs
│       └── borwein_quadratic.rs
└── config/                    # ⚙️ Конфигурация
    └── app_config.rs
```

## 🎯 Domain Layer (Ядро)

### Entities (Сущности)
- **`Algorithm`**: Представляет различные алгоритмы вычисления π
- **`PiComputation`**: Инкапсулирует состояние и результат вычисления

### Value Objects (Объекты-значения)
- **`Digits`**: Типобезопасное представление количества знаков
- **`Precision`**: Точность вычислений в битах

### Services (Доменные сервисы)
- **`PiCalculator`**: Основная бизнес-логика вычисления π

## 🔌 Ports (Порты)

### Input Ports (Входящие порты)
- **`PiComputationUseCase`**: Интерфейс для вычисления π
- **`ProgressReporter`**: Интерфейс для отчета о прогрессе

### Output Ports (Исходящие порты)
- **`PiRepository`**: Интерфейс для выполнения вычислений
- **`FileWriter`**: Интерфейс для записи файлов
- **`UiPresenter`**: Интерфейс для представления UI

## 🔧 Adapters (Адаптеры)

### Input Adapters (Входящие адаптеры)
- **`CliHandler`**: Обработчик CLI команд
- **`ConsoleProgress`**: Отображение прогресса в консоли

### Output Adapters (Исходящие адаптеры)
- **`FileStorage`**: Сохранение в файловую систему
- **`ConsolePresenter`**: Отображение в консоли
- **`AlgorithmRepository`**: Репозиторий алгоритмов

## 🚀 Преимущества новой архитектуры

### 1. **Тестируемость**
```rust
// Легко тестировать с моками
struct MockPiRepository;

#[async_trait]
impl PiRepository for MockPiRepository {
    async fn compute_pi(&self, _: Algorithm, _: Digits, _: Precision) -> Result<String, String> {
        Ok("3.14159...".to_string())
    }
}
```

### 2. **Расширяемость**
```rust
// Легко добавить новый UI адаптер
pub struct WebPresenter;

impl UiPresenter for WebPresenter {
    fn show_welcome(&self) {
        // Отправить JSON через WebSocket
    }
    // ...
}
```

### 3. **Независимость от внешних зависимостей**
- Ядро не зависит от конкретных библиотек UI
- Алгоритмы изолированы в отдельных модулях
- Легко заменить способ сохранения файлов

### 4. **Четкое разделение ответственности**
- **Domain**: Только бизнес-логика
- **Ports**: Только интерфейсы
- **Adapters**: Только реализации

## 📊 Диаграмма зависимостей

```
┌─────────────────┐
│   main.rs       │ ──┐
└─────────────────┘   │
                      ▼
┌─────────────────────────────────────┐
│            Adapters                 │
│  ┌─────────────┐ ┌─────────────────┐│
│  │ CLI Handler │ │ Console         ││
│  │             │ │ Presenter       ││
│  └─────────────┘ └─────────────────┘│
└─────────────┬───────────────────────┘
              │ implements
              ▼
┌─────────────────────────────────────┐
│              Ports                  │
│  ┌──────────────────────────────────┐│
│  │    PiComputationUseCase         ││
│  │    UiPresenter                  ││
│  │    FileWriter                   ││
│  └──────────────────────────────────┘│
└─────────────┬───────────────────────┘
              │ uses
              ▼
┌─────────────────────────────────────┐
│             Domain                  │
│  ┌─────────────┐ ┌─────────────────┐│
│  │ Entities    │ │ Services        ││
│  │ Algorithm   │ │ PiCalculator    ││
│  │ Computation │ │                 ││
│  └─────────────┘ └─────────────────┘│
└─────────────────────────────────────┘
```

## 🛠️ Использование

### Простая настройка
```rust
use pi_stream_terminal::PiStreamApp;

// Автоматическая настройка всех зависимостей
let (use_case, presenter, file_writer, config) = PiStreamApp::setup();

// Использование
let digits = Digits::new(1000)?;
let computation = use_case.compute_pi(Algorithm::ChudnovskyBinarySplitting, digits).await?;
presenter.show_results(&computation);
```

### Кастомная настройка
```rust
// Создание с собственными адаптерами
let algorithm_repo = AlgorithmRepository::new();
let calculator = PiCalculator::new(algorithm_repo);
let use_case = CliHandler::new(calculator);

// Использование с кастомным презентером
let presenter = MyCustomPresenter::new();
```

## 🧪 Тестирование

### Unit тесты доменного слоя
```bash
cargo test domain::
```

### Integration тесты
```bash
cargo test --test integration_tests
```

### Все тесты
```bash
cargo test
```

## 📈 Метрики

- **37 unit тестов** покрывают все слои архитектуры
- **Время компиляции**: ~4.5 сек (cold build)
- **Производительность**: Не изменилась по сравнению с монолитной версией
- **Размер бинарника**: Минимальное увеличение

## 🔄 Миграция

Старый код остается доступным через модуль `pi` для обратной совместимости:

```rust
// Старый способ (deprecated)
use pi_stream_terminal::pi::{compute_pi, PiAlgorithm};

// Новый способ (рекомендуется)
use pi_stream_terminal::{PiStreamApp, Algorithm, Digits};
```

## 🎉 Заключение

Hexagonal Architecture обеспечивает:
- ✅ Лучшую тестируемость
- ✅ Большую гибкость
- ✅ Простоту расширения
- ✅ Четкое разделение ответственности
- ✅ Независимость от фреймворков

Архитектура готова для добавления новых алгоритмов, UI интерфейсов и способов хранения данных без изменения основной логики.
