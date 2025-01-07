all: build doc

build:
	cargo build

check:
	export LSM_SIM_TIME=0 && cargo test -- --test-threads=1

doc:
	cargo doc --no-deps
	rsync --checksum --verbose --archive target/doc/ docs/
	echo '<meta http-equiv=refresh content=0;url=lsm/index.html>' \
		> docs/index.html

