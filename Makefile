test:
	RUSTFLAGS="--cfg ckb_test" cargo test --all -- --nocapture

fmt:
	cargo fmt --all -- --check

clippy:
	cargo clippy --all -- -D warnings -D clippy::clone_on_ref_ptr -D unused_extern_crates -D clippy::enum_glob_use -A clippy::inconsistent_digit_grouping -A clippy::large-digit-groups

.PHONY: test clippy fmt
