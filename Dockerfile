# Use the official Rust image as the base image
FROM rust:latest

# Create a new directory for the app
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Create the src directory
RUN mkdir src

# Create a dummy main file to force Cargo to build the dependencies first
RUN echo 'fn main() {println!("Dummy main file");}' > src/main.rs

# Build only the dependencies to cache them
RUN cargo build --release
RUN rm -f target/release/deps/rust_app*

# Copy the source code
COPY . .

# Build the application
RUN cargo build --release

# Set the startup command to run your compiled binary
CMD ["./target/release/rust-app"]
