.PHONY: build
# .PHONY: run
.PHONY: clean

build:
	cargo build --release
	cd abcf-tm && go build
	mv abcf-tm/abcf target/

clean:
	cargo clean
	rm -rf target
