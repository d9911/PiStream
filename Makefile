.DEFAULT_GOAL := dev
CARGO ?= cargo
.PHONY: dev cli build run start check test lint fmt fmt-check icons

dev:
	$(CARGO) run --locked --bin PiStreamTerminal
cli:
	$(CARGO) run --locked --bin pistream-cli
check: fmt-check lint test
test:
	$(CARGO) test --locked
lint:
	$(CARGO) clippy --locked --all-targets --all-features -- -D warnings
fmt:
	$(CARGO) fmt --all
fmt-check:
	$(CARGO) fmt --all -- --check
icons:
	node scripts/generate-icons.mjs
start: build
ifeq ($(OS),Windows_NT)
SHELL := cmd.exe
.SHELLFLAGS := /C
build:
	powershell.exe -NoProfile -File scripts/build-windows.ps1
run start:
	start "" "dist\windows-x64\PiStreamTerminal.exe"
else
SHELL := /bin/sh
ifeq ($(shell uname -s),Darwin)
build:
	sh scripts/build-macos.sh
run start:
	@test -x dist/PiStreamTerminal.app/Contents/MacOS/PiStreamTerminal || { echo "Run make build first." >&2; exit 1; }
	open dist/PiStreamTerminal.app
else
build:
	$(CARGO) build --release --locked --bin PiStreamTerminal
run start:
	./target/release/PiStreamTerminal
endif
endif
