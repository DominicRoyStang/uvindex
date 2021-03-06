### DEV BUILD STEPS ###
FROM rust:1.48-buster AS setup

# Create service directory
RUN mkdir -p /backend
WORKDIR /backend

# Expose ports in the container
EXPOSE 3000

# BlockedTODO: https://github.com/SergioBenitez/Rocket/milestone/8
# Nightly builds are used for Rocket. Switch to stable builds once milestone 0.5.0 is released.
RUN rustup default nightly

# Source run-time environment variables
ENV WEATHERBIT_API_KEY=$WEATHERBIT_API_KEY
ENV OPENWEATHER_API_KEY=$OPENWEATHER_API_KEY
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=3000

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
EXPOSE 3000

# Source run-time environment variables
ENV WEATHERBIT_API_KEY=$WEATHERBIT_API_KEY
ENV OPENWEATHER_API_KEY=$OPENWEATHER_API_KEY
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=3000

# Install environment dependencies
RUN apt-get update && apt-get install -y ca-certificates libssl-dev

# Copy binary from prod-build stage to this new image
COPY --from=prod-build /backend/target/release/uvindex-backend /bin/.

# Default command, run app
CMD uvindex-backend
