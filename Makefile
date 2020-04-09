test:
	@cargo test
	@cd example && dune clean && dune runtest

utop:
	@cargo test
	@cd example && dune clean && dune utop

clean:
	cargo clean
	cd example && dune clean

.PHONY: test clean
