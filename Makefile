all: build doc

build:
	cargo build

check:
	cargo test

doc:
	cargo doc --no-deps
	rsync --checksum --verbose --archive target/doc/ docs/
	echo '<meta http-equiv=refresh content=0;url=lsm/index.html>' \
		> docs/index.html

