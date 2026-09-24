TEST=test.jpl

all: run

compile:
	cargo build --release

run:
	./target/release/jpl-rs $(TEST) $(FLAGS)

clean:
	cargo clean
