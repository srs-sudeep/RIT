#!/bin/bash
# Development helper script for Rit

set -e

case "$1" in
    build)
        echo "🔨 Building Rit..."
        cargo build
        ;;
    release)
        echo "🚀 Building release..."
        cargo build --release
        ;;
    test)
        echo "🧪 Running tests..."
        cargo test
        ;;
    check)
        echo "✅ Checking code..."
        cargo check
        cargo clippy
        cargo fmt --check
        ;;
    fmt)
        echo "🎨 Formatting code..."
        cargo fmt
        ;;
    docs)
        echo "📚 Generating Rust docs..."
        cargo doc --open
        ;;
    run)
        shift
        cargo run -- "$@"
        ;;
    install)
        echo "📦 Installing rit globally..."
        cargo install --path .
        ;;
    clean)
        echo "🧹 Cleaning build artifacts..."
        cargo clean
        ;;
    *)
        echo "Rit Development Helper"
        echo ""
        echo "Usage: ./scripts/dev.sh <command>"
        echo ""
        echo "Commands:"
        echo "  build    - Build debug version"
        echo "  release  - Build release version"
        echo "  test     - Run all tests"
        echo "  check    - Run cargo check, clippy, and fmt check"
        echo "  fmt      - Format code"
        echo "  docs     - Generate and open Rust documentation"
        echo "  run      - Run rit with arguments (e.g., ./scripts/dev.sh run init)"
        echo "  install  - Install rit globally"
        echo "  clean    - Clean build artifacts"
        ;;
esac

