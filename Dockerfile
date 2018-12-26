# TODO: multistaged
FROM scratch

WORKDIR /repo

COPY target/x86_64-unknown-linux-musl/release/gitmoji-changelog-rust /gitmoji-changelog-rust

ENTRYPOINT ["/gitmoji-changelog-rust"]
CMD ["/repo"]
