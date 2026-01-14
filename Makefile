.PHONY: build test clean install install-service uninstall-service

build:
	cargo build --release
	poetry install

install: build install-service

install-service:
	@mkdir -p ~/.config/systemd/user
	@cp aw-watcher-window-cosmic.service ~/.config/systemd/user/
	@systemctl --user daemon-reload
	@systemctl --user enable aw-watcher-window-cosmic
	@echo "Service installed. Start with: systemctl --user start aw-watcher-window-cosmic"

uninstall-service:
	@systemctl --user stop aw-watcher-window-cosmic || true
	@systemctl --user disable aw-watcher-window-cosmic || true
	@rm -f ~/.config/systemd/user/aw-watcher-window-cosmic.service
	@systemctl --user daemon-reload
	@echo "Service uninstalled."

test:
	poetry run aw-watcher-window-cosmic --help

typecheck:
	poetry run mypy aw_watcher_window_cosmic/ --ignore-missing-imports

clean:
	rm -rf aw_watcher_window_cosmic/__pycache__
	rm -rf venv build dist *.egg-info
	cargo clean
