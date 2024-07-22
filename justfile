# Empty flags to prevent 'frameworks not supported' error: https://github.com/rust-lang/rust/issues/125534
export RUSTFLAGS := ""

# List all targets
default:
    @just --list

# Run shuttle locally
run:
    cargo shuttle --wd service run

# Check code
check:
    cd simulator && cargo check

# Test shuttle deployment
deploy-debug:
    cargo shuttle --debug --wd service deploy

# Serve front locally
serve:
    cd simulator && dx serve --hot-reload

# Build simulator with debug profile
build-debug:
    cd simulator && dx build

# Build simulator with release profile
build-release:
    cd simulator && dx build --release

# Copy compiled assets from the simulator to the service asset folder
copy-assets:
    cp -r simulator/dist/* service/assets/

# Build release version and deploy it to shuttle
deploy: build-release copy-assets
    cargo shuttle --wd service deploy
