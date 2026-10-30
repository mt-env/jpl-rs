TEST=test.jpl
BINARY=target/release/jpl-rs

all: run

compile:
	cargo build --release

$(BINARY):
	cargo build --release

run: $(BINARY)
	./$(BINARY) $(TEST) $(FLAGS)

clean:
	cargo clean
