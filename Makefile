build:
	cargo build --release

install:
	cp ./target/release/aghpb-cli ~/.local/bin

uninstall:
	rm ~/.local/bin/aghpb-cli

clean:
	cargo clean