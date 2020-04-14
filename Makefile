test:
	@cargo test --features=link -- --test-threads=1
	@dune clean --root=./example
	@dune runtest --root=./example

utop:
	@dune clean --root=./example
	@dune utop --root=./example

clean:
	cargo clean
	cd example && dune clean

.PHONY: test clean
