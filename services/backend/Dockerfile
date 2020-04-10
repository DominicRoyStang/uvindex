FROM rust:1.42-stretch as setup

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
# BlockedTODO: https://github.com/terraform-providers/terraform-provider-google/issues/5539
# Update this and related terraform configs when the issue is resolved
ENV ROCKET_PORT=8080

# Copy source code
COPY . .

# Create development build
RUN cargo build

# Run development build
CMD cargo run
