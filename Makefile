TEST=test.jpl

all: run

compile:
	cargo build --release

run: compile
	./target/release/jpl-rs $(TEST) $(FLAGS)

clean:
	cargo clean
