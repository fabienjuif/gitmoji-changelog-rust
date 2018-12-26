package:
	docker pull clux/muslrust
	docker run -v ${PWD}:/volume --rm -t clux/muslrust cargo build --release
	docker build -t gitmoji-changelog-rust .
