FROM rust:1.72 AS build
COPY . .
RUN rustup target add x86_64-unknown-linux-gnu
RUN cargo install --path . --target x86_64-unknown-linux-gnu

FROM alpine:3.16.0 AS runtime
COPY --from=build /usr/local/cargo/bin/proxide /usr/local/bin/proxide

FROM runtime as action
COPY entrypoint.sh /entrypoint.sh

ENTRYPOINT [ /entrypoint.sh ]
