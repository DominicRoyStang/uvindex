FROM rust:1.42-stretch as setup

# Create service directory
RUN mkdir -p /backend
WORKDIR /backend

# Expose ports in the container
EXPOSE 8000

# Set rust to use nightly builds (for Rocket)
RUN rustup default nightly

# Source run-time environment variables
ENV WEATHERBIT_API_KEY=$WEATHERBIT_API_KEY
ENV OPENWEATHER_API_KEY=$OPENWEATHER_API_KEY
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

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
COPY --from=prod-build /backend/target/x86_64-unknown-linux-musl/release/uvindex .
# this works if the uvindex binary is run in the same alpine/stretch version as where it was built
CMD ["./uvindex"]
