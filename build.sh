#!/bin/bash

# Build the WebAssembly target
cargo build --target wasm32-unknown-unknown --release

# Install wasm-bindgen-cli if not already installed
cargo install wasm-bindgen-cli

# Create the dist directory if it doesn't exist
mkdir -p dist

# Generate JavaScript bindings
wasm-bindgen target/wasm32-unknown-unknown/release/trunk-template.wasm --out-dir dist --target web

# Copy the index.html to dist
cp index.html dist/

# Create a simple JavaScript loader
cat > dist/index.js << 'EOL'
async function init() {
    const response = await fetch('trunk-template.wasm');
    const buffer = await response.arrayBuffer();
    const obj = await WebAssembly.instantiate(buffer);
    return obj.instance.exports;
}

init().then(exports => {
    console.log('WASM loaded successfully');
}).catch(err => {
    console.error('Failed to load WASM:', err);
});
EOL

echo "Build completed. You can now serve the 'dist' directory."
