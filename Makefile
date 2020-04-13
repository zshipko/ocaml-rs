test:
	@cargo test --features=link
	@dune clean --root=./example
	@dune runtest --root=./example

utop:
	@dune clean --root=./example
	@dune utop --root=./example

clean:
	cargo clean
	cd example && dune clean

.PHONY: test clean
