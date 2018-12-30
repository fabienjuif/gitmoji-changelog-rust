package:
	@docker build -t fabienjuif/gitmoji-changelog .

quality:
	@rustup component add rustfmt clippy &> /dev/null
	@cargo fmt --all -- --check
	@cargo clippy --quiet --all-targets --all-features -- -D warnings
	@echo "All is ok!"

build-dev:
	@cargo build

ci: quality build-dev
