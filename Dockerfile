# # Use an official Rust runtime as a parent image
# FROM rust:latest as builder

# WORKDIR /usr/src/app

# # Copy the local code into the container
# COPY . .

# # Build your application
# RUN cargo build --release

# # Use a smaller base image for the final container
# FROM debian:buster-slim

# WORKDIR /usr/src/app

# # Copy only the built artifacts from the previous stage
# COPY --from=builder /usr/src/app/target/release/bulk_sms_api .

# # Expose the port your application listens on
# EXPOSE 8080

# # Command to run your application
# CMD ["./bulk_sms_api"]


# FROM rust:latest as builder
# WORKDIR /usr/src/bulk_sms_api
# COPY . .
# # RUN cargo update -p home@0.5.9 --precise ver
# RUN cargo install --path .

# FROM debian:bullseye-slim
# RUN apt-get update && apt-get install -y && rm -rf /var/lib/apt/lists/*
# # RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
# COPY --from=builder /usr/local/cargo/bin/bulk_sms_api /usr/local/bin/bulk_sms_api
# CMD ["bulk_sms_api"]

# Use a minimal Alpine-based image as the base
FROM debian:bullseye-slim

# Set the working directory inside the container
WORKDIR /app

# Create a directory named 'log'
RUN mkdir log

# Create a file named 'log.log' inside the 'log' directory
RUN touch log/sms_gateway.log

# Copy the compiled binary from your local machine into the container
COPY target/release/bulk_sms_api .
COPY .env .

RUN chmod +x bulk_sms_api

# Expose the port your application listens on
EXPOSE 8080

# Command to run your application
CMD ["./bulk_sms_api"]

# docker build -t bulk_sms_api
# docker run -p 8080:8080 bulk_sms_api