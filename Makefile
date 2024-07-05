PHONY: release debug clean

release:
	cargo build --release
	cp target/release/flashout .

debug:
	cargo build
	cp target/debug/flashout .

clean:
	rm -rf /target