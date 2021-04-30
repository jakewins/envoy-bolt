# Building

Use rustup to install (and ensure updated) wasm target:

    rustup toolchain install nightly
    rustup update
    rustup target add wasm32-unknown-unknown --toolchain nightly

Build

    cargo  +nightly build --target wasm32-unknown-unknown --release

Test

    docker-compose up --build