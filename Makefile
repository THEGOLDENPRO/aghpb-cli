build:
	cargo build --release

install:
	cp ./target/release/aghpb-cli /usr/local/bin

clean:
	cargo clean