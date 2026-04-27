

all:
	cargo build --release
	mkdir -p plugins
	cp ./target/debug/*.so ./plugins

run:
	cargo run --release -- ./config.ron
	open image.ppm
