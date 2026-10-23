# jpl-rs

A compiler for JPL (John-Pavel Language) written in Rust, built for CS 4470 (Compilers) at the University of Utah.

## Compiler Status

| Stage           | Status      |
| --------------- | ----------- |
| Lexing          | Done        |
| Parsing         | Done        |
| Typechecking    | Done        |
| IR              | Not started |
| Assembly        | Not started |

## Building

```sh
make compile
```

## Usage

```sh
./target/release/jpl-rs <file.jpl> [mode]
```

or via the Makefile (defaults to `test.jpl`):

```sh
make run TEST=<path/to/file.jpl> FLAGS=<flags>
```

| Flag | Mode                                                                 |
| ---- | -------------------------------------------------------------------- |
| `-l` | Lex and print tokens                                                 |
| `-_` | Lex without printing (performance)                                   |
| `-p` | Parse and print the AST                                              |
| `-t` | Typecheck and print the typed AST                                    |
| `-i` | Emit IR (not yet implemented) (may be deprecated)                    |
| `-s` | Emit assembly (not yet implemented) (may be deprecated)              |

You can only pass one flag at a time.

## Grading

Pushes to `main` run the course autograder via GitHub Actions.

## License

MIT. See [LICENSE](LICENSE).
