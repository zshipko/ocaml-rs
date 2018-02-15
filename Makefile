SRC=$(wildcard src/*.rs)
ARGS=-- -Z unstable-options --pretty=expanded

pretty: $(SRC)
	@cargo rustc $(ARGS)
	@cd examples/rust && cargo rustc $(ARGS)
	@cd examples/ocaml && make pretty
	examples/ocaml/test

test: $(SRC)
	@cargo test
	@cd examples/rust && cargo build
	@cd examples/ocaml && make
	examples/ocaml/test

clean:
	cargo clean
	cd examples/rust && cargo clean
	cd examples/ocaml && make clean

.PHONE: test pretty clean
