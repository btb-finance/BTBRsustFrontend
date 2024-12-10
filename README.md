# BTB Rust Frontend

A Rust-based web application using Yew framework.

## Prerequisites

- Rust (stable version)
- WebAssembly target: `rustup target add wasm32-unknown-unknown`
- basic-http-server: `cargo install basic-http-server`

## Building the Project

1. Clone the repository
2. Run the build script:
   ```bash
   chmod +x build.sh
   ./build.sh
   ```

## Running the Application

1. Start the server:
   ```bash
   basic-http-server -a 127.0.0.1:8081 dist
   ```
2. Open your web browser and navigate to: `http://127.0.0.1:8081`

## Project Structure

- `src/main.rs`: Main application entry point
- `src/routes/`: Application routing
- `src/pages/`: Page components
- `src/components/`: Reusable components

## Development

To make changes to the application:
1. Edit the source files
2. Run `./build.sh` to rebuild
3. Refresh your browser to see the changes
