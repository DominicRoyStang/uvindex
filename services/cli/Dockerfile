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

# Create development build
RUN cargo install cargo-watch
RUN cargo build

# Run development build
CMD cargo watch -x run
