# Use an official Rust runtime as a parent image
FROM rust:1.55-slim-buster

# Set the working directory to /app
WORKDIR /app

# Copy the current directory contents into the container at /app
COPY . /app

# Build the Rust project
RUN cargo build --release

# Make port 8000 available to the world outside this container
EXPOSE 8000

# Run the Rust project when the container launches
CMD ["./target/release/bellatrix-backend"]