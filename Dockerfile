FROM rust:1.82.0 AS builder

WORKDIR /tmp

COPY Cargo.toml Cargo.lock /tmp/
COPY meme_generator /tmp/meme_generator
COPY meme_generator_cli /tmp/meme_generator_cli
COPY meme_generator_py /tmp/meme_generator_py
COPY meme_generator_server /tmp/meme_generator_server
COPY meme_options_derive /tmp/meme_options_derive

RUN cargo build --release --bin server

FROM debian:bookworm-slim AS app

WORKDIR /app

EXPOSE 2233

ENV TZ=Asia/Shanghai \
  MEME_DISABLED_LIST="[]" \
  GIF_MAX_FRAMES=200

COPY --from=builder /tmp/target/release/server /app/

ADD resources/fonts resources/images ~/.meme_generator/resources/

RUN apt-get update \
  && apt-get install -y --no-install-recommends pkg-config libssl-dev libfontconfig1-dev libfreetype6-dev \
  && rm -rf /var/lib/apt/lists/*

RUN nl=$'\n' && echo "\
  [meme] $nl\
  meme_disabled_list = $MEME_DISABLED_LIST $nl\
  [encoder] $nl\
  gif_max_frames = $GIF_MAX_FRAMES $nl\
  [server] $nl\
  host = '0.0.0.0' $nl\
  port = 2233 " > ~/.meme_generator/config.toml

CMD ["/app/server"]
