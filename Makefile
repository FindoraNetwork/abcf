.PHONY: build
# .PHONY: run
.PHONY: clean

build: build_rust build_go

build_rust:
	cargo build --release

build_go:
	cd abcf-tm && go build
	mv abcf-tm/abcf target/


clean:
	cargo clean
	rm -rf target
