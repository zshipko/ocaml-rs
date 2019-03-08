test:
	@cargo test
	@cd example && dune clean && dune exec bin/main.exe

clean:
	cargo clean
	cd example && dune clean

.PHONY: test clean
