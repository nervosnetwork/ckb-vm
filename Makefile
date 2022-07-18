test:
	cargo test --all -- --nocapture

test-asm:
	cargo test --all --features=asm -- --nocapture

test-aot:
	cargo test --all --features=aot -- --nocapture

test-asm-chaos:
	cargo test --all --features=asm,enable-chaos-mode-by-default -- --nocapture

test-aot-chaos:
	cargo test --all --features=aot,enable-chaos-mode-by-default -- --nocapture

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
	-A clippy::suspicious_operation_groupings
clippy:
	cargo clippy --all --features=asm -- $(clippy_rule)
	cd definitions && cargo clippy --all -- $(clippy_rule)

fuzz:
	cargo +nightly fuzz run asm -- -max_total_time=180

ci: fmt clippy test
	git diff --exit-code Cargo.lock

ci-deps:
	echo "skip"
# ci-deps: security-audit check-licenses check-crates
#	git diff --exit-code Cargo.lock

ci-quick: test
	git diff --exit-code Cargo.lock

ci-asm: test-asm
	git diff --exit-code Cargo.lock

ci-aot: test-aot
	git diff --exit-code Cargo.lock

ci-asm-chaos: test-asm-chaos
	git diff --exit-code Cargo.lock

ci-aot-chaos: test-aot-chaos
	git diff --exit-code Cargo.lock

ci-miri:
	echo "skip"
#	rustup component add miri
#	cargo miri setup
#	MIRIFLAGS="-Zmiri-disable-isolation" cargo miri test --all --features=miri-ci

ci-generated: src/machine/aot/aot.x64.compiled.c src/machine/aot/aot.x64.win.compiled.c update-cdefinitions
	git diff --exit-code src/machine/aot/aot.x64.compiled.c src/machine/aot/aot.x64.win.compiled.c src/machine/asm/cdefinitions_generated.h

# For counting lines of code
stats:
	@cargo count --version || cargo +nightly install --git https://github.com/kbknapp/cargo-count
	@cargo count --separator , --unsafe-statistics

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

# Following rules are used to update dynasm compiled files
src/machine/aot/aot.x64.compiled.c: src/machine/aot/aot.x64.c .deps/luajit/src/host/minilua
	.deps/luajit/src/host/minilua .deps/luajit/dynasm/dynasm.lua -o $@ $<

src/machine/aot/aot.x64.win.compiled.c: src/machine/aot/aot.x64.c .deps/luajit/src/host/minilua
	.deps/luajit/src/host/minilua .deps/luajit/dynasm/dynasm.lua -D WIN -o $@ $<

.deps/luajit/src/host/minilua:
	rm -rf .deps/luajit && mkdir -p .deps && \
		git clone https://github.com/LuaJIT/LuaJIT .deps/luajit && \
		cd .deps/luajit && git checkout 0f8a340c8c71fb8f5b8ae7c3ae94bfe81af8f8e8 && \
		make

.PHONY: test clippy fmt fuzz
.PHONY: ci ci-quick ci-all-features ci-cdefinitions
.PHONY: stats security-audit check-licenses check-crates
.PHONY: update-cdefinitions
