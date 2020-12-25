### DEV BUILD STEPS ###
FROM rust:1.48-buster AS setup

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
CMD cargo watch -x 'run -- now -vv'


### PROD BUILD STEPS ###
FROM setup AS prod-build

# Create production build
RUN cargo build --release

# Use a minimalist image
FROM debian:stretch-slim AS prod

# Install environment dependencies
RUN apt-get update && apt-get install -y ca-certificates libssl-dev

# Source run-time environment variables
ENV UVINDEX_BACKEND_PROTOCOL=$UVINDEX_BACKEND_PROTOCOL
ENV UVINDEX_BACKEND_HOST=$UVINDEX_BACKEND_HOST
ENV UVINDEX_BACKEND_PORT=$UVINDEX_BACKEND_PORT

# Copy binary from prod-build stage to this new image
COPY --from=prod-build /cli/target/release/uvindex-cli /bin/.

# Default command, run app
CMD uvindex-cli
