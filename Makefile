test: test-native
test-native: test_link_native
	@dune clean --root=./example
	@dune runtest --root=./example

test-bytecode: test_link_bytecode
	@dune clean --root=./example
	@dune runtest --root=./example

test_link_native:
	@cargo test --features=link-native

test_link_bytecode:
	@cargo test --features=link-bytecode

utop:
	@dune clean --root=./example
	@dune utop --root=./example

clean:
	cargo clean
	cd example && dune clean

.PHONY: test clean
