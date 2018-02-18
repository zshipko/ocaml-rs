## example makefile for building ocaml binary with rust lib
## NB: assumes you use rustup with standard install

RUSTSRC=$(wildcard ../rust/src/*.rs)
CLIENT=../rust/target/debug/libclient.a
ARGS=-- -Z unstable-options --pretty=expanded

build: test.native $(CLIENT)

$(CLIENT): $(RUSTSRC)
	cd ../rust && cargo build

test.native: test.ml $(CLIENT)
	jbuilder build
	cp _build/default/test.exe ./test

pretty: test
	cd ../rust && cargo rustc $(ARGS)

clean:
	rm -rf _build test
	cd ../rust && cargo clean

.PHONY: build clean pretty
