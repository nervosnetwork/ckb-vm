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
	-A clippy::unusual_byte_groupings
clippy:
	cargo clippy --all --features=asm -- $(clippy_rule)
	cd definitions && cargo clippy --all -- $(clippy_rule)

fuzz:
	cargo +nightly fuzz run asm -- -max_total_time=180
	cargo +nightly fuzz run interpreter -- -max_total_time=180
	cargo +nightly fuzz run isa_b -- -max_total_time=180
	cargo +nightly fuzz run isa_m -- -max_total_time=180
	cargo +nightly fuzz run snapshot -- -max_total_time=180
	cargo +nightly fuzz run snapshot2 -- -max_total_time=180

ci: fmt check clippy test
	git diff --exit-code Cargo.lock

ci-asm: test-asm
	git diff --exit-code Cargo.lock

ci-asm-chaos: test-asm-chaos
	git diff --exit-code Cargo.lock

ci-generated: update-cdefinitions
	git diff --exit-code src/machine/asm/cdefinitions_generated.h

update-cdefinitions:
	cargo run --manifest-path=definitions/Cargo.toml --bin generate_asm_constants > src/machine/asm/cdefinitions_generated.h

.PHONY: test clippy fmt fuzz
.PHONY: ci ci-quick ci-all-features ci-cdefinitions
.PHONY: stats security-audit check-licenses check-crates
.PHONY: update-cdefinitions
