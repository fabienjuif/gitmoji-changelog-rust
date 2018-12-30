package:
	@docker build -t fabienjuif/gitmoji-changelog .

tools:
	@rustup component add rustfmt clippy &> /dev/null

quality: tools
	@cargo fmt --all -- --check
	@cargo clippy --quiet --all-targets --all-features -- -D warnings
	@echo "All is ok!"

build-dev: tools
	@cargo build

ci: quality build-dev
