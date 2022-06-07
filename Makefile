all: test

build: src/gen/ffi.rs src/gen/ffi.cc
	cargo build

src/gen/ffi.cc: tpl/gvars.py tpl/ffi.cc
	j2rxx.py -o $@ -g $^

src/gen/ffi.rs: tpl/gvars.py tpl/ffi.rs
	j2rxx.py -o $@ -g $^

test: build
	cargo test -- --nocapture
