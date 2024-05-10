build:
	@cargo build

release:
	@cargo build --release

drun:
	@.././target/debug/auth

run:
	@.././target/release/auth
