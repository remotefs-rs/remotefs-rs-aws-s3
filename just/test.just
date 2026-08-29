# Run all unit, integration, binary, and documentation tests.
[group('test')]
test args="":
  cargo test --all-targets {{ args }}
  cargo test --doc {{ args }}

# Run emulator-backed integration tests.
[group('test')]
integration:
  cargo test --features with-containers -- --test-threads=1

# Generate an LCOV coverage report.
[group('test')]
coverage output="lcov.info":
  cargo llvm-cov --all-targets --lcov --output-path {{ output }}
