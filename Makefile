all: release

release:
	cargo build --release --target x86_64-unknown-linux-musl
unlock:
	@rm ~/.cargo/.package-cache
run_cluster:
	cargo run
clean:
	cargo clean
