test:
	RUSTFLAGS="--cfg ckb_test" cargo test --all -- --nocapture

fmt:
	cargo fmt --all -- --check

clippy:
	cargo clippy --all -- -D warnings -D clone_on_ref_ptr -D unused_extern_crates -D enum_glob_use

.PHONY: test clippy fmt
