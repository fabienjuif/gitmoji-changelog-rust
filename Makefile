package:
	@docker build -t fabienjuif/gitmoji-changelog .

tools:
	@echo "Installing tools"
	@rustup component add rustfmt &> /dev/null
	@rustup component add clippy &> /dev/null
	@echo "Tools: ok!"

quality: tools
	@echo "Running quality"
	@cargo fmt --all -- --check
	@cargo clippy --quiet --all-targets --all-features -- -D warnings
	@echo "Quality: ok!"

build-dev: tools
	@echo "Building"
	@cargo build --quiet
	@echo "Building: ok!"

ci: quality build-dev
