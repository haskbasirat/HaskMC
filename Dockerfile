FROM rust:1-alpine3.23 AS builder
ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN apk add --no-cache musl-dev \
    # Required for git-version
    git

WORKDIR /haskmc
COPY . /haskmc

RUN rustup show active-toolchain || rustup toolchain install
RUN rustup component add rustfmt

# build release
RUN --mount=type=cache,sharing=private,target=/haskmc/target \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --release -p haskmc && cp target/release/haskmc ./haskmc.release

FROM alpine:3.24

COPY --from=builder /haskmc/haskmc.release /bin/haskmc
COPY --from=builder /haskmc/LICENSE /usr/share/licenses/haskmc/LICENSE
COPY --from=builder /haskmc/FORK_NOTICE.md /usr/share/licenses/haskmc/FORK_NOTICE.md
COPY --from=builder /haskmc/THIRD_PARTY_NOTICES.md /usr/share/licenses/haskmc/THIRD_PARTY_NOTICES.md

LABEL org.opencontainers.image.title="HaskMC" \
      org.opencontainers.image.source="https://github.com/haskbasirat/HaskMC" \
      org.opencontainers.image.licenses="GPL-3.0-only"

# set workdir to /haskmc, this is required to influence the PWD environment variable
# it allows for bind mounting the server files without overwriting the haskmc
# executable (without requiring an `docker cp`-ing the binary to the host folder)
WORKDIR /haskmc

RUN apk add --no-cache libgcc && chown 2613:2613 .

ENV RUST_BACKTRACE=1
EXPOSE 25565
USER 2613:2613
ENTRYPOINT [ "/bin/haskmc" ]
HEALTHCHECK CMD nc -z 127.0.0.1 25565 || exit 1
