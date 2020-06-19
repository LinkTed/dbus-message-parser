#!/bin/bash -e


URL="https://github.com/mozilla/grcov/releases/latest/download/grcov-linux-x86_64.tar.bz2"
# Download and unpack grcov
curl -L "$URL" | tar jxf -

# Set environment variables
export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code \
	-Coverflow-checks=off -Zpanic_abort_tests"

# Create code coverage (gcno and gcda files)
cargo test --verbose --all

# Pack the gcno and gcda files into a zip file
.travis/zip.sh

# Convert gcno and gcda files into lcov
./grcov ccov.zip -s . -t lcov --branch -o lcov.info \
	--ignore-not-existing --ignore "/*" --ignore "tests/*" \
	--excl-line "#\[derive\(" --excl-br-line "#\[derive\("

# Upload code coverage
bash <(curl -s https://codecov.io/bash) -f lcov.info
