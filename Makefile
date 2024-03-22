test: test-rust test-ocaml

test-rust:
	@cargo test --features=link -- --test-threads=1

test-ocaml:
	@dune clean --root=test
	@dune runtest --root=test --force --no-buffer

test-book:
	@cargo clean
	@cargo build
	@mdbook test doc -L ./target/debug/deps

build-book:
	@mdbook build doc

utop:
	@dune utop --root=test

clean:
	cargo clean
	dune clean --root=test
	mdbook clean doc || :

publish-sys:
	cd sys && cargo package && cargo publish && sleep 20

publish:
	cd derive && cargo package && cargo publish && sleep 20
	cd build && cargo package && cargo publish && sleep 20
	cargo package && cargo publish
	make deploy-book

deploy-book: build-book
	@echo "====> deploying to github"
	git worktree remove /tmp/ocaml-rs-book || :
	git worktree add /tmp/ocaml-rs-book gh-pages
	mdbook build doc
	rm -rf /tmp/ocaml-rs-book/*
	cp -rp doc/book/* /tmp/ocaml-rs-book/
	cd /tmp/ocaml-rs-book && \
		git update-ref -d refs/heads/gh-pages && \
    	git add -A && \
    	git commit -m "deployed on $(shell date) by ${USER}" && \
    	git push origin gh-pages -f && \
		rm -r /tmp/ocaml-rs-book

.PHONY: test clean
