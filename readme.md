# Rust Customer Support automation API

This is a Rust API project that exposes an endpoint and processes JSON data.

## Project Structure

The project has the following files:

- `Cargo.toml`: The manifest file for Rust's package manager, Cargo.
- `Dockerfile`: A text document that contains all the commands to assemble an image.
- `docker-compose.yml`: A YAML file defining services, networks, and volumes for docker-compose.
- `init_pgvector.sql`: An SQL file for initializing the pgvector.
- `src/`: The source directory that contains the Rust source code.
- `request_example.sh`: A shell script for testing the API.

## Setup

Follow these steps to set up the project:

1. Run Docker Compose: `docker-compose -d`
2. Build the project: `cargo build`
3. Run the project: `cargo run`
4. Set the OpenAI API key: `export OPENAI_API_KEY=sk-<your-api-key>`

## Testing

You can test the API using the provided shell script: `bash request_example.sh`
