all: test

build: src/wrapper.rs
	cargo build

src/wrapper.rs: tpl/wrapper.rs tpl/eigen_map.rs
	j2render.py -o $@ $<

test: build
	cargo test -- --nocapture
