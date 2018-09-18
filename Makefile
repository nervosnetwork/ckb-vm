test:
	RUSTFLAGS="--cfg ckb_test" cargo test --all -- --nocapture

fmt:
	cargo fmt --all -- --check

clippy:
	cargo clippy --all -- -D warnings -D clone_on_ref_ptr -D unused_extern_crates -D enum_glob_use -A inconsistent_digit_grouping -A large-digit-groups

ci: fmt clippy test
	git diff --exit-code Cargo.lock

.PHONY: test clippy fmt ci
