### DEV BUILD STEPS ###
FROM rust:1.42-stretch AS setup

# Create service directory
RUN mkdir -p /backend
WORKDIR /backend

# Expose ports in the container
EXPOSE 8080

# Set rust to use nightly builds (for Rocket)
RUN rustup default nightly

# Source run-time environment variables
ENV WEATHERBIT_API_KEY=$WEATHERBIT_API_KEY
ENV OPENWEATHER_API_KEY=$OPENWEATHER_API_KEY
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080

# Copy source code
COPY . .

FROM setup AS dev
# Create development build
RUN cargo install cargo-watch
RUN cargo build

# Run development build
CMD cargo watch -x run


### PROD BUILD STEPS ###
FROM setup AS prod-build

# Create production build
RUN cargo build --release

# Use a minimalist image
FROM debian:stretch-slim AS prod

# Expose ports in the container
EXPOSE 8080

# Source run-time environment variables
ENV WEATHERBIT_API_KEY=$WEATHERBIT_API_KEY
ENV OPENWEATHER_API_KEY=$OPENWEATHER_API_KEY
ENV ROCKET_ADDRESS=0.0.0.0
# BlockedTODO: https://github.com/terraform-providers/terraform-provider-google/issues/5539
# Update port and related terraform configs when the issue is resolved
ENV ROCKET_PORT=8080

# Install environment dependencies
RUN apt-get update && apt-get install -y ca-certificates libssl-dev

# Copy binary from prod-build stage to this new image
COPY --from=prod-build /backend/target/release/uvindex-backend /bin/.

# Default command, run app
CMD uvindex-backend
