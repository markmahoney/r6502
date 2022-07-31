all: build_bin run

build_bin:
	cd test_bin && make

build:
	cargo build

run:
	cargo run
