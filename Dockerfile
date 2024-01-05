# syntax=docker/dockerfile:1.4

FROM alpine:3.16 as build-environment

ARG TARGETARCH
WORKDIR /opt

RUN apk add clang lld curl build-base linux-headers git \
    && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rustup.sh \
    && chmod +x ./rustup.sh \
    && ./rustup.sh -y

RUN [[ "$TARGETARCH" = "arm64" ]] && echo "export CFLAGS=-mno-outline-atomics" >> $HOME/.profile || true

WORKDIR /opt/foxar
COPY . .

RUN --mount=type=cache,target=/root/.cargo/registry --mount=type=cache,target=/root/.cargo/git --mount=type=cache,target=/opt/foxar/target \
    source $HOME/.profile && cargo build --release \
    && mkdir out \
    && mv target/release/spark out/spark \
    && mv target/release/probe out/probe \
    && mv target/release/shuttle out/shuttle \
    && mv target/release/pilot out/pilot \
    && strip out/spark \
    && strip out/probe \
    && strip out/pilot \
    && strip out/shuttle;

FROM docker.io/frolvlad/alpine-glibc:alpine-3.16_glibc-2.34 as foxar-client

RUN apk add --no-cache linux-headers git

COPY --from=build-environment /opt/foxar/out/spark /usr/local/bin/spark
COPY --from=build-environment /opt/foxar/out/probe /usr/local/bin/probe
COPY --from=build-environment /opt/foxar/out/shuttle /usr/local/bin/shuttle
COPY --from=build-environment /opt/foxar/out/pilot /usr/local/bin/pilot

RUN adduser -Du 1000 foxar

ENTRYPOINT ["/bin/sh", "-c"]


LABEL org.label-schema.build-date=$BUILD_DATE \
    org.label-schema.name="Foxar" \
    org.label-schema.description="Foxar" \
    org.label-schema.url="https://getfoxar.sh" \
    org.label-schema.vcs-ref=$VCS_REF \
    org.label-schema.vcs-url="https://github.com/bchainhub/foxar.git" \
    org.label-schema.vendor="Foxar-rs" \
    org.label-schema.version=$VERSION \
    org.label-schema.schema-version="1.0"
