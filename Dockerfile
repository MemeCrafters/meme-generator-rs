FROM rust:1.88.0 AS builder

WORKDIR /tmp

COPY \
  Cargo.toml Cargo.lock \
  meme_generator \
  meme_generator_cli \
  meme_generator_core \
  meme_generator_memes \
  meme_generator_node \
  meme_generator_py \
  meme_generator_server \
  meme_generator_utils \
  meme_options_derive \
  /tmp/

RUN export MEME_IMAGES_DIR=/app/resources/images \
  && cargo build --release --bin server

FROM debian:bookworm-slim AS app

EXPOSE 2233

ENV TZ=Asia/Shanghai \
  MEME_HOME=/data \
  RUST_LOG=info

VOLUME /data

WORKDIR /app

COPY --from=builder /tmp/target/release/server /app/server
COPY resources/images /app/resources/images/
COPY resources/fonts /usr/share/fonts/meme-fonts/

RUN apt-get update \
  && apt-get install -y --no-install-recommends openssl fontconfig \
  && fc-cache -fv \
  && rm -rf /var/lib/apt/lists/*

CMD ["/app/server"]
