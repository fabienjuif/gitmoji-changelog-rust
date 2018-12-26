FROM clux/muslrust as builder

COPY . /volume/
RUN cargo build --release

FROM scratch

WORKDIR /repo

COPY --from=builder /volume/target/x86_64-unknown-linux-musl/release/gitmoji-changelog-rust /gitmoji-changelog-rust

ENTRYPOINT ["/gitmoji-changelog-rust"]
CMD ["/repo"]
