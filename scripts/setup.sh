#!/bin/bash
# Towa Setup Script for Linux/macOS
# Author: Towa Contributors

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

print_header() {
    echo -e "${CYAN}============================================${NC}"
    echo -e "${CYAN}   Towa Environment Setup (Unix)${NC}"
    echo -e "${CYAN}============================================${NC}"
    echo
}

print_success() {
    echo -e "${GREEN}[+]${NC} $1"
}

print_error() {
    echo -e "${RED}[!]${NC} $1"
}

print_info() {
    echo -e "${CYAN}[*]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[!]${NC} $1"
}

check_environment() {
    print_info "Checking environment..."
    echo

    # Check if towa-cli is built
    if [ -f "src/rust/target/release/towa-cli" ]; then
        src/rust/target/release/towa-cli check
    elif [ -f "src/rust/target/debug/towa-cli" ]; then
        src/rust/target/debug/towa-cli check
    else
        print_warning "Towa CLI not found. Building from source..."
        build_rust
        if [ -f "src/rust/target/release/towa-cli" ]; then
            src/rust/target/release/towa-cli check
        fi
    fi
}

install_all() {
    print_info "Installing all development tools..."
    echo

    install_c
    install_rust
    build_all

    echo
    print_success "Installation complete!"
    print_info "Run './scripts/setup.sh check' to verify installation"
}

install_c() {
    print_info "Installing C/C++ toolchain..."
    echo

    OS=$(uname -s)

    case "$OS" in
        Linux*)
            if command -v apt-get &> /dev/null; then
                print_info "Ubuntu/Debian detected"
                sudo apt-get update
                sudo apt-get install -y build-essential cmake git pkg-config
                print_success "C/C++ toolchain installed"

            elif command -v dnf &> /dev/null; then
                print_info "Fedora/RHEL detected"
                sudo dnf groupinstall -y "Development Tools"
                sudo dnf install -y cmake git
                print_success "C/C++ toolchain installed"

            elif command -v pacman &> /dev/null; then
                print_info "Arch Linux detected"
                sudo pacman -S --noconfirm base-devel cmake git
                print_success "C/C++ toolchain installed"

            else
                print_error "Unsupported Linux distribution"
                exit 1
            fi
            ;;

        Darwin*)
            print_info "macOS detected"
            if ! command -v brew &> /dev/null; then
                print_info "Installing Homebrew..."
                /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
            fi

            print_info "Installing tools via Homebrew..."
            brew install gcc cmake git
            print_success "C/C++ toolchain installed"
            ;;

        *)
            print_error "Unsupported operating system: $OS"
            exit 1
            ;;
    esac
}

install_rust() {
    print_info "Installing Rust toolchain..."
    echo

    if command -v rustc &> /dev/null; then
        print_success "Rust is already installed. Updating..."
        rustup update
    else
        print_info "Installing Rust via rustup..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

        # Source cargo env
        source "$HOME/.cargo/env"

        print_success "Rust installed successfully"
        print_warning "Please restart your terminal or run: source \$HOME/.cargo/env"
    fi
}

install_cepton() {
    print_info "Configuring Cepton SDK..."
    echo

    if [ -f "src/rust/target/release/towa-cli" ]; then
        src/rust/target/release/towa-cli cepton
    else
        print_warning "Please download Cepton SDK from:"
        echo "    https://www.cepton.com/downloads"
        echo
        print_info "Then set environment variable:"
        echo "    export CEPTON_SDK_PATH=/opt/cepton_sdk"
        echo "    export PATH=\$PATH:\$CEPTON_SDK_PATH/bin"
        echo
        print_info "Add these lines to your ~/.bashrc or ~/.zshrc"
    fi
}

build_all() {
    print_info "Building Towa tools..."
    echo

    build_c
    build_rust
}

build_c() {
    print_info "Building C tools..."
    cd src/c

    mkdir -p build
    cd build

    cmake ..
    cmake --build . --config Release

    cd ../../..
    print_success "C tools built successfully"
}

build_rust() {
    print_info "Building Rust tools..."
    cd src/rust

    cargo build --release

    cd ../..
    print_success "Rust tools built successfully"
}

show_help() {
    echo "Towa Setup Script"
    echo
    echo "Usage: ./setup.sh [command]"
    echo
    echo "Commands:"
    echo "  check      - Check current environment status"
    echo "  install    - Install all development tools"
    echo "  c          - Install C/C++ toolchain only"
    echo "  rust       - Install Rust toolchain only"
    echo "  cepton     - Configure Cepton SDK"
    echo "  build      - Build all Towa tools"
    echo "  help       - Show this help message"
    echo
    echo "Examples:"
    echo "  ./setup.sh check"
    echo "  ./setup.sh install"
    echo "  ./setup.sh build"
}

# Main script logic
print_header

case "$1" in
    check)
        check_environment
        ;;
    install)
        install_all
        ;;
    c)
        install_c
        ;;
    rust)
        install_rust
        ;;
    cepton)
        install_cepton
        ;;
    build)
        build_all
        ;;
    help|--help|-h)
        show_help
        ;;
    "")
        show_help
        ;;
    *)
        print_error "Unknown command: $1"
        show_help
        exit 1
        ;;
esac
