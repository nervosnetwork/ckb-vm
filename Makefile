test:
	cargo test --all -- --nocapture

test-asm:
	cargo test --all --features=asm -- --nocapture

test-asm-chaos:
	cargo test --all --features=asm,enable-chaos-mode-by-default -- --nocapture

check:
	cargo check --all --all-targets --all-features

cov:
	cargo clean
	cargo build --tests --all --features=asm
	for file in `find target/debug/ -maxdepth 1 -executable -type f`; do mkdir -p "target/cov/$$(basename $$file)"; kcov --exclude-pattern=/.cargo,/usr/lib,tests --verify "target/cov/$$(basename $$file)" "$$file"; done

fmt:
	cargo fmt --all -- --check
	cd definitions && cargo fmt ${VERBOSE} --all -- --check

clippy_rule = -D warnings \
	-D clippy::clone_on_ref_ptr \
	-D clippy::enum_glob_use \
	-A clippy::collapsible-else-if \
	-A clippy::upper_case_acronyms \
	-A clippy::unusual_byte_groupings \
	-A clippy::inconsistent_digit_grouping \
	-A clippy::large_digit_groups \
	-A clippy::suspicious_operation_groupings \
	-A clippy::unnecessary_cast
clippy:
	cargo clippy --all --features=asm -- $(clippy_rule)
	cd definitions && cargo clippy --all -- $(clippy_rule)

fuzz:
	cargo +nightly fuzz run asm -- -max_total_time=180
	cargo +nightly fuzz run isa_a -- -max_total_time=180

ci: fmt check clippy test
	git diff --exit-code Cargo.lock

ci-deps: security-audit check-licenses check-crates
	git diff --exit-code Cargo.lock

ci-asm: test-asm
	git diff --exit-code Cargo.lock

ci-asm-chaos: test-asm-chaos
	git diff --exit-code Cargo.lock

ci-generated: update-cdefinitions
	git diff --exit-code src/machine/asm/cdefinitions_generated.h

# Use cargo-deny to audit Cargo.lock for crates with security vulnerabilities
security-audit:
	@cargo deny --version || cargo install cargo-deny
	@cargo deny check --hide-inclusion-graph --show-stats advisories sources

# Use cargo-deny to check licenses for all dependencies.
check-licenses:
	@cargo deny --version || cargo install cargo-deny
	@cargo deny check --hide-inclusion-graph --show-stats licenses

# Use cargo-deny to check specific crates, detect and handle multiple versions of the same crate and wildcards version requirement.
check-crates:
	@cargo deny --version || cargo install cargo-deny
	@cargo deny check --hide-inclusion-graph --show-stats bans

update-cdefinitions:
	cargo run --manifest-path=definitions/Cargo.toml --bin generate_asm_constants > src/machine/asm/cdefinitions_generated.h

.PHONY: test clippy fmt fuzz
.PHONY: ci ci-quick ci-all-features ci-cdefinitions
.PHONY: stats security-audit check-licenses check-crates
.PHONY: update-cdefinitions
