CC=cargo
FMT=fmt

OPTIONS=

default: fmt
	$(CC) build --all-features
	@make example

fmt:
	$(CC) fmt --all

check:
	$(CC) test --all

example:
	$(CC) build --example main

clean:
	$(CC) clean
