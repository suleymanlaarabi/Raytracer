IMAGE := image.ppm
CONFIG := config.ron

all:
	cargo build --release
	mkdir -p plugins
	cp ./target/release/*.so ./plugins

run:
	cargo run --release -- ./$(CONFIG) --sfml

close-image:
	@wmctrl -l | grep "$(IMAGE)" | while read id rest; do \
		wmctrl -ic "$$id" || true; \
	done || true

watch:
	@echo "Watching $(CONFIG)..."
	@while true; do \
		inotifywait -qq -e close_write,moved_to,create $(CONFIG); \
		$(MAKE) close-image; \
		cargo run --release -- ./$(CONFIG) || continue; \
		if [ -f "$(IMAGE)" ]; then \
			open ./$(IMAGE) >/dev/null 2>&1 & \
		else \
			echo "Erroor: $(IMAGE) not found"; \
		fi; \
	done
