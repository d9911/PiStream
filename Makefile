.DEFAULT_GOAL := dev
CARGO ?= cargo
export CARGO
ifeq ($(OS),Windows_NT)
PYTHON ?= python
else
PYTHON ?= python3
endif
.PHONY: help doctor setup dev cli build local run start check test lint fmt fmt-check icons
help doctor setup dev cli build local run start check test lint fmt fmt-check:
	"$(PYTHON)" scripts/project.py $@
icons:
	node scripts/generate-icons.mjs
