#!/bin/sh
set -eu
project_dir=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
cd "$project_dir"
if pgrep -x PiStreamTerminal >/dev/null; then
    echo "Закрой PiStreamTerminal перед пересборкой. Готовое приложение: make run." >&2
    exit 1
else
    process_check=$?
    if [ "$process_check" -ne 1 ]; then
        echo "Не удалось проверить запущенные процессы; сборка остановлена." >&2
        exit 1
    fi
fi
export MACOSX_DEPLOYMENT_TARGET=11.0
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$project_dir/target}"
cargo build --release --locked --bin PiStreamTerminal
mkdir -p dist
stage=$(mktemp -d "$project_dir/dist/.pistream-package.XXXXXX")
trap 'rm -rf "$stage"' EXIT HUP INT TERM
app="$stage/PiStreamTerminal.app"
mkdir -p "$app/Contents/MacOS" "$app/Contents/Resources"
cp "$CARGO_TARGET_DIR/release/PiStreamTerminal" "$app/Contents/MacOS/PiStreamTerminal"
cp packaging/macos/Info.plist "$app/Contents/Info.plist"
cp assets/icons/PiStreamTerminal.icns "$app/Contents/Resources/PiStreamTerminal.icns"
cp README.md "$app/Contents/Resources/README.md"
if [ -f LICENSE ]; then cp LICENSE "$app/Contents/Resources/LICENSE"; fi
printf 'APPL????' > "$app/Contents/PkgInfo"
plutil -lint "$app/Contents/Info.plist"
codesign --force --sign "${PISTREAM_SIGNING_IDENTITY:--}" "$app"
codesign --verify --strict "$app"
# Replace only this script's generated bundle, after validation has passed.
if [ -d dist/PiStreamTerminal.app ]; then mv dist/PiStreamTerminal.app "$stage/previous.app"; fi
mv "$app" dist/PiStreamTerminal.app
architecture=$(uname -m)
ditto -c -k --norsrc --keepParent dist/PiStreamTerminal.app "dist/PiStreamTerminal-macos-$architecture.zip"
echo "Готово: $project_dir/dist/PiStreamTerminal.app"
