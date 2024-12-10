# BTB Rust Frontend

A Rust-based web application using Yew framework.

## Prerequisites

- Rust (stable version 1.74.0 or higher)
- WebAssembly target: `rustup target add wasm32-unknown-unknown`
- wasm-bindgen-cli (v0.2.99): `cargo install wasm-bindgen-cli --version 0.2.99`
- basic-http-server: `cargo install basic-http-server`
- make (for build automation)

## Project Setup

1. Clone the repository
2. Install dependencies:
   ```bash
   rustup target add wasm32-unknown-unknown
   cargo install wasm-bindgen-cli --version 0.2.99
   cargo install basic-http-server
   ```

## Building and Running

1. Build the project:
   ```bash
   make build
   ```

2. Start the development server:
   ```bash
   make serve
   ```

3. Open your web browser and navigate to: `http://127.0.0.1:8081`

## Project Structure

- `src/`: Source code directory
  - `main.rs`: Main application entry point and root component
  - `routes/`: Application routing configuration
  - `pages/`: Page components
  - `components/`: Reusable UI components
- `build.rs`: Rust build script for WebAssembly compilation
- `Makefile`: Build automation
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
2. Run `make build` to rebuild
3. Run `make serve` to start the development server
4. Refresh your browser to see the changes

To clean the build:
```bash
make clean
```

## Troubleshooting

If you encounter issues:

1. Version Mismatches:
   - Ensure wasm-bindgen-cli version matches the one in Cargo.toml (currently 0.2.99)
   - Run `make clean` before rebuilding if you update versions

2. Build Issues:
   - Check that all prerequisites are installed
   - Verify that the WebAssembly target is installed: `rustup target list | grep wasm32`
   - Make sure you have make installed

3. Runtime Issues:
   - Clear browser cache
   - Check browser console (F12) for errors
   - Verify that the server is running on the correct port
   - Try using a private/incognito window

## Recent Changes

1. Build System:
   - Replaced shell scripts with Rust build script (build.rs)
   - Added Makefile for build automation
   - Improved build process reliability

2. WebAssembly Setup:
   - Updated wasm-bindgen to version 0.2.99
   - Added proper WebAssembly initialization
   - Streamlined build process

3. Project Structure:
   - Simplified main.rs for initial "Hello World" display
   - Added proper module organization
   - Updated Cargo.toml with correct dependencies

## Next Steps

- Add more UI components
- Implement routing for multiple pages
- Add state management
- Integrate with backend services
- Add CSS styling and UI framework
- Set up testing infrastructure
