build:
	cargo build
run:
	./target/debug/copycat
all:
	cargo build && ./target/debug/copycat