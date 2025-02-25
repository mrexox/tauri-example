TAURI_ARCH = $(shell uname -m | sed 's/amd64/x86_64/' | sed 's/arm64/aarch64/')

OUTPUT_PATH ?= sidecar

build:
	CGO_ENABLED=0 go build -ldflags "-s -w" -o $(OUTPUT_PATH)

install: OUTPUT_PATH = ../bin/sidecar-$(TAURI_ARCH)-apple-darwin
install: build

