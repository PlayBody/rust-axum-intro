test:
	cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"

app:
	cargo watch -q -c -w src/ -x run

run:
	cargo run

build:
	cargo build