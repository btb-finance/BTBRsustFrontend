# BTB Rust Frontend

A Rust-based web application using Yew framework.

## Prerequisites

- Rust (stable version 1.74.0 or higher)
- WebAssembly target: `rustup target add wasm32-unknown-unknown`
- wasm-bindgen-cli (v0.2.99): `cargo install wasm-bindgen-cli --version 0.2.99`
- basic-http-server: `cargo install basic-http-server`

## Project Setup

1. Clone the repository
2. Install dependencies:
   ```bash
   rustup target add wasm32-unknown-unknown
   cargo install wasm-bindgen-cli --version 0.2.99
   cargo install basic-http-server
   ```

## Building the Project

1. Make the build script executable:
   ```bash
   chmod +x build.sh
   ```
2. Run the build script:
   ```bash
   ./build.sh
   ```

## Running the Application

1. Start the server:
   ```bash
   basic-http-server -a 127.0.0.1:8081 dist
   ```
2. Open your web browser and navigate to: `http://127.0.0.1:8081`

## Project Structure

- `src/main.rs`: Main application entry point and root component
- `src/routes/`: Application routing configuration
- `src/pages/`: Page components
- `src/components/`: Reusable UI components
- `build.sh`: Build script for compiling to WebAssembly
- `index.html`: HTML template with WebAssembly loader
- `Cargo.toml`: Rust dependencies and project configuration

## Key Dependencies

- `yew`: Main web framework (v0.20)
- `yew-router`: Routing (v0.17.0)
- `wasm-bindgen`: WebAssembly bindings (v0.2.99)
- `web-sys`: Web APIs
- `js-sys`: JavaScript interop

## Development

To make changes to the application:
1. Edit the source files
2. Run `./build.sh` to rebuild
3. Refresh your browser to see the changes

## Troubleshooting

If you encounter issues:

1. Version Mismatches:
   - Ensure wasm-bindgen-cli version matches the one in Cargo.toml (currently 0.2.99)
   - Run `cargo clean` before rebuilding if you update versions

2. Build Issues:
   - Check that all prerequisites are installed
   - Verify that the WebAssembly target is installed: `rustup target list | grep wasm32`
   - Make sure build.sh has execute permissions

3. Runtime Issues:
   - Clear browser cache
   - Check browser console (F12) for errors
   - Verify that the server is running on the correct port
   - Try using a private/incognito window

## Recent Changes

1. WebAssembly Setup:
   - Updated wasm-bindgen to version 0.2.99
   - Added proper WebAssembly initialization in index.html
   - Modified build script to use wasm-bindgen-cli

2. Project Structure:
   - Simplified main.rs for initial "Hello World" display
   - Added proper module organization
   - Updated Cargo.toml with correct dependencies

3. Development Environment:
   - Added comprehensive build script
   - Configured development server
   - Added proper WebAssembly loading

## Next Steps

- Add more UI components
- Implement routing for multiple pages
- Add state management
- Integrate with backend services
- Add CSS styling and UI framework
- Set up testing infrastructure
