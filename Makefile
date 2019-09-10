test:
	cargo test --all -- --nocapture

test-all-features:
	cargo test --all --features=asm -- --nocapture

check:
	cargo check --all --all-targets --all-features

cov:
	cargo clean
	cargo build --tests --all --features=asm
	for file in `find target/debug/ -maxdepth 1 -executable -type f`; do mkdir -p "target/cov/$$(basename $$file)"; kcov --exclude-pattern=/.cargo,/usr/lib,tests --verify "target/cov/$$(basename $$file)" "$$file"; done

fmt:
	cargo fmt --all -- --check
	cd definitions && cargo fmt ${VERBOSE} --all -- --check

clippy:
	cargo clippy --all --features=asm -- -D warnings -D clippy::clone_on_ref_ptr -D clippy::enum_glob_use -A clippy::inconsistent_digit_grouping -A clippy::large-digit-groups
	cd definitions && cargo clippy --all -- -D warnings -D clippy::clone_on_ref_ptr -D clippy::enum_glob_use -A clippy::inconsistent_digit_grouping -A clippy::large-digit-groups

ci: fmt clippy test
	git diff --exit-code Cargo.lock

ci-quick: test
	git diff --exit-code Cargo.lock

ci-all-features: test-all-features
	git diff --exit-code Cargo.lock

ci-generated: src/machine/aot/aot.x64.compiled.c src/machine/aot/aot.x64.win.compiled.c update-cdefinitions
	git diff --exit-code src/machine/aot/aot.x64.compiled.c src/machine/aot/aot.x64.win.compiled.c src/machine/asm/cdefinitions_generated.h

# For counting lines of code
stats:
	@cargo count --version || cargo +nightly install --git https://github.com/kbknapp/cargo-count
	@cargo count --separator , --unsafe-statistics

# Use cargo-audit to audit Cargo.lock for crates with security vulnerabilities
# expecting to see "Success No vulnerable packages found"
security-audit:
	@cargo audit --version || cargo install cargo-audit
	@cargo audit

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
		cd .deps/luajit && git checkout v2.1 && \
		make

.PHONY: test clippy fmt
.PHONY: ci ci-quick ci-all-features ci-cdefinitions
.PHONY: stats security-audit
.PHONY: update-cdefinitions
