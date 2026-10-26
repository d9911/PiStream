# PiStreamTerminal: установка и локальная сборка

Версия берётся из `Cargo.toml`; bundle ID остаётся `org.d9911.pistreamterminal`.
GUI и `pistream-cli` входят в один пакет. Для готового приложения не нужны Rust,
Python, Make, MSYS2 или открытый терминал. Эти инструменты нужны только сборщику.

## Исходное состояние

| ОС / архитектура | Прежняя команда | Инструменты сборки | Прежний результат | Исправляемый пробел |
| --- | --- | --- | --- | --- |
| macOS arm64 / Intel | `make build` | Rust, Xcode CLT | `.app`, ZIP | Оформленный DMG и проверка установленной копии |
| Windows x64 GNU | `scripts/build-windows.ps1` | Rust GNU, MSYS2 | EXE, ZIP | Установщик, ярлыки, удаление, проверка DLL |
| Ubuntu arm64 / amd64 | `make build` | Rust, C toolchain, GUI headers | Cargo release | DEB, зависимости, меню и защита активного процесса |

Это описание исходных скриптов, а не подтверждение работы всех архитектур.

## Единые команды

| Команда | Действие |
| --- | --- |
| `make help` | Команды и каталог результата |
| `make doctor` | Диагностика без изменения системы |
| `make setup` | Rust target, rustfmt/Clippy; изолированные DMG-инструменты на Mac |
| `make dev` | Прежний GUI через Cargo; цель по умолчанию |
| `make cli` | Прежний консольный интерфейс |
| `make build` | GUI + CLI, нативный установщик и ZIP на Mac/Windows |
| `make local` | Сборка текущих исходников и открытие установщика |
| `make run` | Существующая release-копия без пересборки |
| `make start` | Сборка и открытие release-копии |
| `make check` | Существующие fmt, Clippy, тесты |

На Windows все действия доступны без GNU Make: `./package.ps1 help`,
`./package.ps1 setup`, `./package.ps1 local`, `./package.ps1 run`.
`-PythonExe` принимает путь к нативному Python. Обёртки делегируют одному
самостоятельному `scripts/project.py`; обращений к соседним checkout нет.

## macOS

Нужны Rust не ниже `rust-version` проекта, Python 3.11+, Xcode Command Line Tools,
`make`, `m4`, `diff`. Инструменты DMG устанавливаются в `build/package-tools`,
системный Python не изменяется.

```sh
make setup
make doctor
make local
```

Для текущей arm64-сборки результаты:

- `dist/PiStreamTerminal.app`;
- `dist/PiStreamTerminal-0.2.0-macos-arm64.dmg`;
- `dist/PiStreamTerminal-macos-arm64.zip`.

В окне DMG перетащите приложение в Applications, подтвердите замену при обновлении
и запускайте установленную копию. Перед заменой закройте GUI и завершите CLI.
Упаковка не запускает вычислений и не меняет файлы результатов.
CLI находится в `PiStreamTerminal.app/Contents/MacOS/pistream-cli`.

Проверяются подпись .app, подпись копии в DMG и checksum образа. По умолчанию
подпись локальная ad-hoc. `PISTREAM_SIGNING_IDENTITY` позволяет выбрать имеющийся
Developer ID, но notarization и публикация не выполняются. arm64 не является
universal; Intel требует отдельной нативной сборки и проверки.

## Windows x64 GNU / MSYS2

Нужны нативный Python 3.11+, Rust GNU, Inno Setup 6.3+ и MSYS2 MinGW64.
Для закреплённого `gmp-mpfr-sys 1.7.1` используйте требования его документации:

```sh
# В MSYS2 MinGW64; подготовка инструментов разработчика, не готового приложения:
pacman -S --needed diffutils m4 make mingw-w64-x86_64-gcc
```

В обычном PowerShell с нативными Rust и Python:

```powershell
$env:PATH = "C:\msys64\mingw64\bin;C:\msys64\usr\bin;" + $env:PATH
.\package.ps1 setup
.\package.ps1 doctor
.\package.ps1 local
```

`doctor` требует `gcc`, `make`, `m4`, `diff`, `windres` и `objdump`/`dumpbin`.
При нестандартной установке задайте `ISCC` полным путём к `ISCC.exe`.
Сборщик использует `x86_64-pc-windows-gnu`; MSVC вместо GNU для GMP не подставляется.

Результаты: `dist/windows-x64/PiStreamTerminal.exe`, `pistream-cli.exe` рядом,
`dist/PiStreamTerminal-windows-x64.zip` и
`dist/PiStreamTerminal-0.2.0-windows-x86_64-setup.exe`.
Реальные DLL-импорты обоих EXE и включённых DLL проверяются рекурсивно. Отсутствующая
несистемная DLL останавливает упаковку. Не предполагается, что установлен MSYS2.

Установщик работает для текущего пользователя: меню, необязательный ярлык рабочего
стола, штатное удаление. Mutex обоих процессов запрещает замену/удаление во время
работы. Автозапуск и службы не создаются; пользовательские результаты не удаляются.
Нативная сборка, проверка DLL и установка на Windows пока не подтверждены.

## Ubuntu 24.04

Пакеты сборки, использованные в изолированной проверочной Ubuntu:

```sh
sudo apt-get update
sudo apt-get install build-essential m4 diffutils pkg-config python3 python3-venv dpkg-dev binutils xdg-utils libx11-dev libxi-dev libxtst-dev libxrandr-dev libxcursor-dev libxinerama-dev libxkbcommon-dev libxkbcommon-x11-dev libgl1-mesa-dev libegl1-mesa-dev libwayland-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libssl-dev libdbus-1-dev
make setup
make doctor
make build
sudo apt install ./dist/pistreamterminal_0.2.0_arm64.deb
```

На amd64 имя пакета заканчивается на `_amd64.deb`; сборка должна выполняться
нативно на этой архитектуре. Проверенная среда — Ubuntu 24.04 arm64 с X11/Mesa/Xvfb;
остальные версии Ubuntu, amd64 и полноценный Wayland-сеанс требуют проверки.

Меню использует `.desktop` и штатную иконку. `/usr/bin/pistreamterminal` открывает
`/opt/pistreamterminal/PiStreamTerminal`; `/usr/bin/pistream-cli` запускает CLI.
ELF-зависимости вычисляет `dpkg-shlibdeps`; дополнительные динамически загружаемые
библиотеки GUI и xdg-desktop-portal указаны в metadata. Для системного диалога файлов
нужен работающий desktop portal. Обновление/удаление активной установленной копии
блокируется, после её закрытия файлы пользовательских результатов сохраняются.

## GMP, пробелы и Cargo target

GMP/MPFR/MPC собираются из исходников закреплённого `gmp-mpfr-sys`, штатные C-проверки
не отключаются. В проверенной macOS-сборке `otool -L` обоих EXE не содержит внешних
GMP/MPFR/MPC dylib. На Windows дополнительно проверяются фактические DLL каждого EXE.

POSIX libtool не поддерживает пробелы в физическом каталоге сборки. Когда выбранный
Cargo target содержит пробелы, помощник предварительно заполняет штатный native cache
GMP из временного каталога без пробелов, используя версию и features из locked-графа.
Основная сборка остаётся в выбранном `CARGO_TARGET_DIR`, её точные executable paths
читаются из JSON Cargo. По умолчанию native cache хранится только в `build/native-cache`.
Можно задать собственный `GMP_MPFR_SYS_CACHE`; отключённый кэш при пробелах отклоняется.
На Windows используется временный свободный SUBST-диск для того же target, без изменения
существующих дисков. Эта ветвь требует нативной Windows-проверки.

При ошибке компиляции предыдущие опубликованные в локальный dist пакеты сохраняются.
Сборка не завершает пользовательские процессы; установленное приложение не требует
сборочного кэша. Настройки/история GUI остаются в памяти, сохранённые результаты —
в выбранном пользователем месте.

## Источники требований

- [gmp-mpfr-sys 1.7.1: сборка и native cache](https://docs.rs/gmp-mpfr-sys/1.7.1/gmp_mpfr_sys/index.html).
- [MSYS2: разделение окружений и runtime](https://www.msys2.org/docs/environments/).
- [Inno Setup AppMutex: защита установки и удаления](https://jrsoftware.org/ishelp/topic_setup_appmutex.htm).
- Локальные источники: `Cargo.toml`, `Cargo.lock`, `packaging/app.json`, `scripts/project.py`.


## Выполненная проверка 26 октября 2026

- macOS arm64: fmt, Clippy с `-D warnings`, 52 существующих теста; один тяжёлый тест штатно игнорируется. DMG/.app/ZIP собраны, подписи и checksum проверены.
- Копия из read-only DMG в отдельном каталоге с пробелами запущена через LaunchServices: окно, иконка, ресурсы и состояние «Готово к вычислению» проверены. Автоматического вычисления нет.
- Установленный CLI запущен с системным PATH, без Cargo: явно выбран Binary Splitting и 10 знаков, получено `3.1415926536`; файлов результатов не создано.
- Подставленная ошибка Cargo остановила сборку; хэши прежних установщиков и manifest остались прежними.
- Ubuntu 24.04 arm64: fmt, строгий Clippy и те же 52 теста, один ignored. Сборка выполнена из `/work/PiStreamTerminal QA`, с пробелом в физическом target.
- DEB установлен в изолированной Ubuntu; GUI открыт обычным пользователем с системным PATH, `.desktop` прошёл `desktop-file-validate`. CLI независимо получил те же 10 знаков.
- Обновление и удаление активной копии отказались менять файлы; после закрытия оба действия прошли. Пользовательский файл с результатом сохранился. `ldd` обоих EXE не показал отсутствующих библиотек или внешних GMP/MPFR/MPC.
- Windows, Intel Mac, Ubuntu amd64, полноценный Wayland и Citrix не проверены.

Дополнительная macOS-проверка завершена: полноценный DMG построен из каталога с пробелами и отдельного `CARGO_TARGET_DIR` с пробелами; GUI и CLI получены именно в заданном target. Штатные C-проверки GMP/MPFR/MPC при предварительном заполнении кэша прошли без отключения.

Итоговая упаковка включает эту публичную инструкцию. Дополнительно проверен откат при искусственной ошибке продвижения файлов: прежние .app, установщики и manifest восстановлены; output-symlink отклоняется до замены. Повторная проверка активного экземпляра выполняется непосредственно перед продвижением.
