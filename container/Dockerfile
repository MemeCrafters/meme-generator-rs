FROM debian:buster

RUN apt-get update && apt-get install -y \
    clang \
    gcc \
    g++ \
    curl \
    build-essential \
    software-properties-common \
    libfontconfig1-dev \
    libssl-dev \
    pkg-config \
    python3 \
    python3-pip \
    ninja.build

RUN apt-get install -y \
    dh-autoreconf \
    libcurl4-gnutls-dev \
    libexpat1-dev \
    gettext \
    libz-dev

ARG GIT_VERSION=2.47.0
ARG GIT_URL=https://github.com/git/git/archive/refs/tags/v$GIT_VERSION.tar.gz
RUN curl -sL $GIT_URL | tar xz -C /opt && \
    cd /opt/git-$GIT_VERSION && \
    make -j 12 && make prefix=/usr/local install

ENV SKIA_NINJA_COMMAND=/usr/bin/ninja \
    PATH=/usr/local/bin:$PATH
