TEST=test.jpl

all: run

compile:
	cargo build --release

run:
	cargo run --release -- $(TEST) $(FLAGS)

clean:
	cargo clean
