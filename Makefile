build:
	cargo build --release

install:
	cp ./target/release/aghpb-cli /usr/local/bin

uninstall:
	rm /usr/local/bin/aghpb-cli

clean:
	cargo clean