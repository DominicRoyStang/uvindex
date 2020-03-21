FROM rust:1.42-stretch as setup

# Create service directory
RUN mkdir -p /cli
WORKDIR /cli

# Source run-time environment variables
ENV UVINDEX_BACKEND_PROTOCOL=$UVINDEX_BACKEND_PROTOCOL
ENV UVINDEX_BACKEND_HOST=$UVINDEX_BACKEND_HOST
ENV UVINDEX_BACKEND_PORT=$UVINDEX_BACKEND_PORT

# Copy source code
COPY . .


FROM setup AS dev
# Create development build
RUN cargo install cargo-watch
RUN cargo build

# Run development build
CMD cargo watch -x run


FROM setup AS prod-build
# Create production build
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl


# Run production build
FROM scratch AS prod
COPY --from=prod-build /cli/target/x86_64-unknown-linux-musl/release/uvindex .
# this works if the uvindex binary is run in the same alpine/stretch version as where it was built
CMD ["./uvindex"]
