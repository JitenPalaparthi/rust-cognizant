# Use the official Rust image as the base image for building the app
FROM rust:1.81 as builder

# Set the working directory for the app
WORKDIR /usr/src/app

# # Copy the Cargo.toml and Cargo.lock files to the container
# COPY Cargo.toml Cargo.lock ./

# # Create a build-only dummy file to cache dependencies
# RUN touch src/main.rs

# # Build the dependencies and cache them
# RUN cargo build --release

# Now copy the actual source code
COPY . .

# Build the app in release mode
RUN cargo build --release

# Use the same base image for runtime
FROM rust:1.81-slim

# Install necessary dependencies
RUN apt-get update && apt-get install -y \
    libssl-dev \
    libpq-dev \
    ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Create a user to run the application
RUN groupadd -g 1000 appuser && \
    useradd -m -u 1000 -g appuser appuser

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/app/target/release/my_app /usr/local/bin/my_app

# Set the working directory for the app
WORKDIR /usr/src/app

# Switch to a non-root user
USER appuser

# Expose the port that the Actix Web application will run on
EXPOSE 8084

# Command to run the application
CMD ["my_app"]