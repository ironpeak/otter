FROM clux/muslrust:stable AS chef
USER root
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY ./Cargo.toml ./Cargo.toml
COPY ./src/ ./src/
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY ./src/ ./src/
RUN cargo build --release --bin otter
RUN mkdir /out && mv /app/target/x86_64-unknown-linux-musl/release/otter /out/otter

FROM golang:alpine AS csharp
RUN apk add --update --no-cache \
    bash \ 
    icu-libs \ 
    krb5-libs \ 
    libgcc \ 
    libintl \ 
    libssl1.1 \ 
    libstdc++ \ 
    wget \ 
    zlib
RUN wget https://dot.net/v1/dotnet-install.sh -O install.sh \
    && chmod +x install.sh \
    && ./install.sh
RUN mv /root/.dotnet /out

FROM golang:alpine AS golang
RUN apk add --update --no-cache git make
RUN git clone https://github.com/sonatype-nexus-community/nancy.git
RUN cd nancy && make build
RUN mkdir /out && mv nancy/nancy /out/nancy

FROM clux/muslrust:stable AS rust
RUN cargo install cargo-audit
RUN mkdir /out && mv /root/.cargo/bin/cargo-audit /out/cargo-audit

FROM alpine:latest
# csharp
ENV PATH /otter/bin/dotnet:$PATH
ENV DOTNET_CLI_TELEMETRY_OPTOUT 1
RUN apk add --update --no-cache icu-libs
COPY --from=csharp /out /otter/bin/dotnet
RUN dotnet --version
# golang
ENV PATH /otter/bin/nancy:$PATH
RUN apk add --update --no-cache go
COPY --from=golang /out /otter/bin/nancy
RUN nancy --version
# javascript
RUN apk add --update --no-cache npm \
    && npm install --global yarn
RUN node --version \
    && npm --version \
    && yarn --version
# python
RUN apk add --update --no-cache python3 py3-pip \
    && pip3 install pip-audit
RUN pip-audit --version
# rust
ENV PATH /otter/bin/cargo_audit:$PATH
COPY --from=rust /out /otter/bin/cargo_audit
RUN cargo-audit --version
# otter
RUN addgroup -S otter && adduser -S otter -G otter
COPY --from=builder /out /usr/local/bin/
USER otter
CMD ["/usr/local/bin/otter"]
