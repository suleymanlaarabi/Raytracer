

all:
	cargo build
	mkdir -p plugins
	cp ./target/debug/*.so ./plugins

run: all
	cargo run -- ./config.ron
	open image.ppm
