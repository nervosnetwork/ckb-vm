test:
	cargo test --all -- --nocapture

fmt:
	cargo fmt --all -- --check

clippy:
	cargo clippy --all -- -D warnings -D clone_on_ref_ptr -D unused_extern_crates -D enum_glob_use -A inconsistent_digit_grouping -A large-digit-groups

ci: fmt clippy test
	git diff --exit-code Cargo.lock

ci-quick: test
	git diff --exit-code Cargo.lock

# For counting lines of code
stats:
	@cargo count --version || cargo +nightly install --git https://github.com/kbknapp/cargo-count
	@cargo count --separator , --unsafe-statistics

# Use cargo-audit to audit Cargo.lock for crates with security vulnerabilities
# expecting to see "Success No vulnerable packages found"
security-audit:
	@cargo audit --version || cargo install cargo-audit
	@cargo audit

.PHONY: test clippy fmt
.PHONY: ci ci-quick
.PHONY: stats security-audit
