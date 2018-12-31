# gitmoji-changelog-rust
> Do you use gitmoji? Then generate your changelog with this app!

This is a Rust version of [gitmoji-changelog](https://github.com/frinyvonnick/gitmoji-changelog).

<br />
<p style="text-align: center" align="center">
  <a href="https://circleci.com/gh/fabienjuif/gitmoji-changelog-rust/tree/master">
    <img src="https://img.shields.io/circleci/project/github/fabienjuif/gitmoji-changelog-rust/master.svg" />
  </a>
  <a href="https://crates.io/crates/gitmoji-changelog">
    <img src="https://img.shields.io/crates/v/gitmoji-changelog.svg" />
  </a>
  <a href="https://hub.docker.com/r/fabienjuif/gitmoji-changelog">
    <img src="https://img.shields.io/badge/docker--image-fabienjuif%2Fgitmoji--changelog-blue.svg" />
    <img src="https://img.shields.io/microbadger/image-size/fabienjuif%2Fgitmoji-changelog.svg" />
  </a>
  <br />
  [<a href="https://docs.rs/gitmoji-changelog/">documentation</a>]
  [<a href="https://github.com/fabienjuif/gitmoji-changelog-rust">repository</a>]
</p>
<br />

## Why
I was sad about the space it takes in a Docker container with the NodeJS version and I am learning Rust: so I was curious and it helps me have a little Rust CLI project to play with.

## Try it
### With Docker 🐳!
```sh
## try it
docker run --rm -v ${PWD}:/repo fabienjuif/gitmoji-changelog

## to see which options you can use:
docker run --rm -v ${PWD}:/repo fabienjuif/gitmoji-changelog --help
```

### With cargo
```sh
## install it
cargo install gitmoji-changelog

# maybe you should reset your env here (relaunch your terminal or type `zsh` (or `bash`))

## try it
gitmoji-changelog .

## to see which options you can use:
gitmoji-changelog --help
```

## Roadmap
For now, this project is just a test I do.
But if I (or you) want to push this further:
 - [x] List commits betweens 2 hashes
 - [x] Group commits by "code"
 - [x] Group commits by version
 - [x] Create a markdown
 - [x] Create an incremental markdown
 - [x] Detect which tags to start from
 - [x] Add author
 - [ ] Group similar commits
 - [ ] Links to github

This is a lot of work and I this is surely not worh it!

## Commands
This project use a `Makefile`, here are the main targets:
  - `package`: build the docker image
  - `ci`: build the project (dev mode) and check clippy and rustfmt

You can still use cargo if you want to, eg building the release version with: `cargo build --release`
