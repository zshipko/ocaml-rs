test:
	@cargo test
	@cd example && dune clean && dune exec bin/main.exe

utop:
	@cargo test
	@cd example && dune clean && dune utop

clean:
	cargo clean
	cd example && dune clean

.PHONY: test clean
