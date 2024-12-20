# Step 1: Build the Rust Application
FROM rust:1.81 as builder

# Create and set the working directory
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock to leverage Docker cache
COPY Cargo.toml Cargo.lock ./

# Build dependencies first to cache them
# RUN cargo build --release && rm -f target/release/deps/my_project*

# Copy the rest of the application code
COPY . .

# Build the final release binary
RUN cargo build --release

# Step 2: Set up the final image
FROM debian:bullseye-slim

# Install necessary dependencies
RUN apt-get update && apt-get install -y \
    libpq-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/my_app /usr/local/bin/my_app

# Expose the port the application will run on
EXPOSE 8084

# Set the command to run the application
CMD ["my_app"]