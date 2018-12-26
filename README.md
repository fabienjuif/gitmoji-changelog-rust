# gitmoji-changelog-rust
Rust version of [gitmoji-changelog](https://github.com/frinyvonnick/gitmoji-changelog)

## Why
I was sad about the space it takes in a Docker container with the NodeJS version and I am learning Rust: so I was curious and it helps me have a little Rust CLI project to play with.

## Roadmap
For now, this project is just a test I do.
But if I (or you) want to push this further:
 - [ ] List commits betweens 2 hashes
 - [ ] Create a markdown
 - [ ] Create an incremental markdown
 - [ ] Detect which tags to start from
 - [ ] Add author
 - [ ] Group similar commits

This is a lot of work and I this is surely not worh it!

## Commands
Create the Docker image with:
```sh
make
```

All other commands pass through Cargo.
