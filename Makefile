test:
	@cargo test --features=link -- --test-threads=1
	@dune clean --root=test
	@dune runtest --root=test

utop:
	@dune clean --root=test
	@dune utop --root=test

clean:
	cargo clean
	dune clean --root=test

publish:
	cd sys && cargo package && cargo publish
	cd derive && cargo package && cargo publish
	cargo package && cargo publish

.PHONY: test clean
