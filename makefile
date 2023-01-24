build:
	cargo build --release
run: build
	cargo run --release -- -c config.ron > example.ppm
clean:
	cargo clean
	rm example.ppm