# NB: This is not a production-grade Dockerfile.

#################
## build stage ##
#################
FROM rust:1-slim-bullseye AS builder

# Download crates-io index and fetch dependency code.
# This step avoids needing to spend time on every build downloading the index
# which can take a long time within the docker context. Docker will cache it.
RUN USER=root cargo new --bin schary-mono
WORKDIR /schary-mono
RUN USER=root cargo new --bin migration
COPY Cargo.toml Cargo.toml
COPY migration/Cargo.toml migration/Cargo.toml
RUN cargo build --release
RUN rm src/*.rs
RUN rm migration/src/*.rs
# RUN cargo fetch

# copy app files
COPY src src
COPY migration/src migration/src
COPY templates templates
COPY Config.yaml Config.yaml

# compile app
RUN cargo build --release

###############
## run stage ##
###############
FROM debian:bullseye-slim
WORKDIR /app

# copy server binary from build stage
COPY --from=builder /schary-mono/target/release/schary-mono schary-mono

# set user to non-root unless root is required for your app
USER 1001

# indicate what port the server is running on
EXPOSE 30000

# run server
CMD [ "/app/schary-mono" ]