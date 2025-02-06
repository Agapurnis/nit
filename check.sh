echo "=== [NIGHTLY] CLIPPY === " && cargo +nightly clippy \
&& echo "=== [NIGHTLY] TEST (DEBUG) === " && cargo +nightly test \
&& echo "=== [NIGHTLY] TEST (RELEASE) === " && cargo +nightly test --release \
&& echo "=== [STABLE] CLIPPY === " && cargo clippy \
&& echo "=== [STABLE] TEST (DEBUG) === " && cargo test \
&& echo "=== [STABLE] TEST (RELEASE) === " && cargo test --release
