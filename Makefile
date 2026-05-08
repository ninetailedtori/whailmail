# SPDX-FileCopyrightText: 2026-Present ninetailedtori <ninetailedtori@uwu.gal>
# SPDX-FileContributor: WhailMail contributors
#
# SPDX-License-Identifier: GPL-3.0-or-later

SHELL := /bin/bash
NPM := /bin/pnpm

.DEFAULT_GOAL := build

#--- Profiles

.PHONY: api-only self-hosted
api-only:
	@echo "Building API-only mode..."
	cd backend && cargo build --release
	cd frontend && $(NPM) run build

self-hosted:
	@echo "Building self-hosted mode..."
	cd backend && cargo build --release --all-features
	cd frontend && $(NPM) run build

#--- Targets

.PHONY: linux-x86-64 linux-arm64 macos-x86-64 macos-arm64 windows-x86-64
.PHONY: desktop-linux desktop-macos desktop-windows
.PHONY: mobile-ios mobile-android

linux-x86-64:
	cd backend && cargo build --release --target x86_64-unknown-linux-gnu

linux-arm64:
	cd backend && cargo build --release --target aarch64-unknown-linux-gnu

macos-x86-64:
	cd backend && cargo build --release --target x86_64-apple-darwin

macos-arm64:
	cd backend && cargo build --release --target aarch64-apple-darwin

windows-x86-64:
	cd backend && cargo build --release --target x86_64-pc-windows-msvc

desktop-linux: linux-x86-64
	cd frontend && $(NPM) run electron:build
	@echo "[OK] Desktop Linux ready"

desktop-macos: macos-universal
	cd frontend && $(NPM) run electron:build
	@echo "[OK] Desktop macOS ready"

desktop-windows: windows-x86-64
	cd frontend && $(NPM) run electron:build
	@echo "[OK] Desktop Windows ready"

mobile-ios:
	cd frontend && npx capacitor sync ios && npx capacitor build ios
	@echo "[OK] Mobile iOS ready in Xcode"

mobile-android:
	cd frontend && npx capacitor sync android && npx capacitor build android
	@echo "[OK] Mobile Android ready in Android Studio"

#--- Build

.PHONY: build build-frontend build-backend
build: api-only
	@echo "[OK] Build complete"

build-frontend:
	cd frontend && $(NPM) run build
	@echo "[OK] Frontend built"

build-backend: build-frontend
	cd backend && cargo build --release
	@echo "[OK] Backend built"

#--- Install

PREFIX ?= /usr/local
INSTALL_BIN := $(PREFIX)/bin
INSTALL_ETC := /etc/whailmail
INSTALL_VAR := /var/lib/whailmail
INSTALL_SHARE := $(PREFIX)/share/whailmail
SYSTEMD_DIR ?= /etc/systemd/system

.PHONY: install install-linux install-macos install-windows
.PHONY: install-config install-systemd uninstall

install: install-$(shell uname -s | tr '[:upper:]' '[:lower:]')
	@echo "[OK] Installed to $(PREFIX)"

install-linux install-darwin: build-backend build-frontend
	@echo "Installing WhailMail to $(PREFIX)..."
	mkdir -p $(INSTALL_BIN) $(INSTALL_ETC) $(INSTALL_VAR) $(INSTALL_SHARE)/templates
	install -m 755 backend/target/release/whailmail-api $(INSTALL_BIN)/
	install -m 644 config/* $(INSTALL_ETC)/
	install -m 644 templates/* $(INSTALL_SHARE)/templates/
	@echo "[OK] Binary: $(INSTALL_BIN)/whailmail-api"
	@echo "[OK] Config: $(INSTALL_ETC)/"
	@echo "[OK] Data: $(INSTALL_VAR)/"

install-windows: build-backend build-frontend
	@echo "Installing WhailMail to $(PREFIX)..."
	mkdir -p "$(PREFIX)/bin" "$(PREFIX)/etc/whailmail" "$(PREFIX)/share/whailmail/templates"
	install -m 755 backend/target/release/whailmail-api.exe "$(PREFIX)/bin/"
	cp config/* "$(PREFIX)/etc/whailmail/"
	cp templates/* "$(PREFIX)/share/whailmail/templates/"
	@echo "[OK] Binary: $(PREFIX)/bin/whailmail-api.exe"
	@echo "[OK] Config: $(PREFIX)/etc/whailmail/"

install-systemd: build-backend
	@echo "Installing systemd service..."
	mkdir -p $(SYSTEMD_DIR)
	install -m 644 contrib/whailmail.service $(SYSTEMD_DIR)/
	systemctl daemon-reload
	@echo "[OK] Run: sudo systemctl enable --now whailmail"

install-config:
	@echo "Installing config to $(INSTALL_ETC)..."
	mkdir -p $(INSTALL_ETC)
	install -m 644 config/* $(INSTALL_ETC)/
	@echo "[OK] Config installed to $(INSTALL_ETC)/"

uninstall:
	@echo "Uninstalling WhailMail..."
	rm -f $(INSTALL_BIN)/whailmail-api $(INSTALL_BIN)/whailmail-api.exe
	rm -rf $(INSTALL_VAR)
	@echo "[OK] Binary and data removed"
	@echo "Config files remain at $(INSTALL_ETC)/ (run 'sudo rm -rf $(INSTALL_ETC)' to remove)"

#--- Development

.PHONY: dev dev-backend dev-frontend run-server
dev-backend:
	cd backend && cargo watch -x 'run --release'

dev-frontend:
	cd frontend && $(NPM) run dev

run-server: build-backend
	./backend/target/release/whailmail-api

dev:
	@echo "Run in separate terminals:"
	@echo "  Terminal 1: make dev-backend"
	@echo "  Terminal 2: make dev-frontend"
	@echo "  Terminal 3: make run-server"

#--- Cleanup

.PHONY: clean
clean:
	rm -rf frontend/dist backend/assets/frontend
	cd backend && cargo clean
	cd frontend && $(NPM) run clean
	@echo "[OK] Clean complete"
