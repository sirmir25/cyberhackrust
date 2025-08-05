#!/bin/bash

# ===================================================================
#  ██████╗██╗   ██╗██████╗ ███████╗██████╗ ██╗  ██╗ █████╗  ██████╗██╗  ██╗
# ██╔════╝╚██╗ ██╔╝██╔══██╗██╔════╝██╔══██╗██║  ██║██╔══██╗██╔════╝██║ ██╔╝
# ██║      ╚████╔╝ ██████╔╝█████╗  ██████╔╝███████║███████║██║     █████╔╝ 
# ██║       ╚██╔╝  ██╔══██╗██╔══╝  ██╔══██╗██╔══██║██╔══██║██║     ██╔═██╗ 
# ╚██████╗   ██║   ██████╔╝███████╗██║  ██║██║  ██║██║  ██║╚██████╗██║  ██╗
#  ╚═════╝   ╚═╝   ╚═════╝ ╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝╚═╝  ╚═╝
# 
#                        CYBERHACK SETUP SCRIPT
#                      Advanced Hacking Simulator
# ===================================================================

# Colors for terminal output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
WHITE='\033[1;37m'
NC='\033[0m' # No Color

# ASCII Art and effects
print_banner() {
    clear
    echo -e "${CYAN}"
    echo "    ╔══════════════════════════════════════════════════════════════╗"
    echo "    ║                    CYBERHACK INSTALLER                       ║"
    echo "    ║                                                              ║"
    echo "    ║    [████████████████████████████████████████████████████]    ║"
    echo "    ║                                                              ║"
    echo "    ║          > Initializing hack environment...                 ║"
    echo "    ║          > Checking system vulnerabilities...               ║"
    echo "    ║          > Establishing secure connection...                 ║"
    echo "    ╚══════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
    sleep 2
}

# Hacking-style loading animation
loading_animation() {
    local text="$1"
    echo -ne "${GREEN}[+] ${text}"
    for i in {1..10}; do
        echo -ne "."
        sleep 0.1
    done
    echo -e " ${GREEN}[OK]${NC}"
}

# Error message with hacking style
error_msg() {
    echo -e "${RED}[!] ERROR: $1${NC}"
    echo -e "${YELLOW}[!] OPERATION FAILED - SYSTEM BREACH DETECTED${NC}"
}

# Success message
success_msg() {
    echo -e "${GREEN}[✓] $1${NC}"
}

# Warning message
warning_msg() {
    echo -e "${YELLOW}[!] WARNING: $1${NC}"
}

# Info message
info_msg() {
    echo -e "${BLUE}[i] $1${NC}"
}

# Check if running as root
check_root() {
    if [[ $EUID -eq 0 ]]; then
        warning_msg "Running as root detected. Some operations may require elevated privileges."
    fi
}

# System requirements check
check_system() {
    info_msg "Scanning target system..."
    
    # Check OS
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        echo -e "${GREEN}[✓] Linux detected${NC}"
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        echo -e "${GREEN}[✓] macOS detected${NC}"
    elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "cygwin" ]]; then
        echo -e "${GREEN}[✓] Windows/WSL detected${NC}"
    else
        warning_msg "Unknown operating system: $OSTYPE"
    fi
    
    # Check architecture
    ARCH=$(uname -m)
    echo -e "${GREEN}[✓] Architecture: $ARCH${NC}"
    
    # Check available disk space
    AVAILABLE_SPACE=$(df . | awk 'NR==2 {print $4}')
    if [[ $AVAILABLE_SPACE -lt 1000000 ]]; then # Less than 1GB
        warning_msg "Low disk space detected. Recommend at least 1GB free space."
    else
        success_msg "Sufficient disk space available"
    fi
}

# Check for Rust installation
check_rust() {
    info_msg "Checking for Rust installation..."
    
    if command -v rustc &> /dev/null; then
        RUST_VERSION=$(rustc --version)
        success_msg "Rust found: $RUST_VERSION"
        
        if command -v cargo &> /dev/null; then
            CARGO_VERSION=$(cargo --version)
            success_msg "Cargo found: $CARGO_VERSION"
            return 0
        else
            error_msg "Cargo not found but Rust is installed"
            return 1
        fi
    else
        error_msg "Rust not found on system"
        return 1
    fi
}

# Install Rust if not present
install_rust() {
    info_msg "Initiating Rust installation protocol..."
    echo -e "${YELLOW}"
    echo "╔════════════════════════════════════════════════════════════════╗"
    echo "║                    RUST INSTALLATION REQUIRED                 ║"
    echo "║                                                                ║"
    echo "║  Rust is required to compile and run CYBERHACK.               ║"
    echo "║  This will download and install Rust using rustup.            ║"
    echo "║                                                                ║"
    echo "║  Do you want to proceed with installation? [y/N]               ║"
    echo "╚════════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
    
    read -p "Continue? " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        loading_animation "Downloading Rust installer"
        
        if command -v curl &> /dev/null; then
            curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        elif command -v wget &> /dev/null; then
            wget -qO- https://sh.rustup.rs | sh -s -- -y
        else
            error_msg "Neither curl nor wget found. Please install Rust manually from https://rustup.rs/"
            exit 1
        fi
        
        # Source the cargo environment
        source "$HOME/.cargo/env"
        success_msg "Rust installation completed"
    else
        error_msg "Installation cancelled by user"
        exit 1
    fi
}

# Check for additional dependencies
check_dependencies() {
    info_msg "Scanning for additional dependencies..."
    
    # Check for git (useful for updates)
    if command -v git &> /dev/null; then
        success_msg "Git found"
    else
        warning_msg "Git not found - updates may be limited"
    fi
    
    # Check for common build tools
    if command -v gcc &> /dev/null || command -v clang &> /dev/null; then
        success_msg "C compiler found"
    else
        warning_msg "No C compiler found - some dependencies may fail to build"
    fi
    
    # Check for pkg-config (sometimes needed for Rust crates)
    if command -v pkg-config &> /dev/null; then
        success_msg "pkg-config found"
    else
        info_msg "pkg-config not found (may be needed for some dependencies)"
    fi
}

# Create necessary directories
setup_directories() {
    info_msg "Creating game directories..."
    
    # Create save directory
    mkdir -p "$HOME/.cyberhack/saves"
    mkdir -p "$HOME/.cyberhack/configs"
    mkdir -p "$HOME/.cyberhack/logs"
    mkdir -p "$HOME/.cyberhack/mods" # For future mod support
    
    success_msg "Game directories created"
}

# Build the project
build_project() {
    info_msg "Compiling CYBERHACK..."
    echo -e "${YELLOW}"
    echo "╔════════════════════════════════════════════════════════════════╗"
    echo "║                     COMPILING SOURCE CODE                     ║"
    echo "║                                                                ║"
    echo "║  This may take several minutes depending on your system...    ║"
    echo "╚════════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
    
    # Check if Cargo.toml exists
    if [[ ! -f "Cargo.toml" ]]; then
        error_msg "Cargo.toml not found. Are you in the correct directory?"
        exit 1
    fi
    
    # Build in release mode for better performance
    loading_animation "Fetching dependencies"
    if cargo fetch; then
        success_msg "Dependencies fetched successfully"
    else
        error_msg "Failed to fetch dependencies"
        exit 1
    fi
    
    loading_animation "Compiling (this may take a while)"
    if cargo build --release; then
        success_msg "Compilation successful!"
    else
        error_msg "Compilation failed"
        echo -e "${YELLOW}[!] Try running: cargo build --release --verbose${NC}"
        exit 1
    fi
}

# Create launch script
create_launcher() {
    info_msg "Creating launcher script..."
    
    cat > cyberhack << 'EOF'
#!/bin/bash

# CYBERHACK Game Launcher
GAME_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
EXECUTABLE="$GAME_DIR/target/release/cyberhack"

# Check if compiled binary exists
if [[ ! -f "$EXECUTABLE" ]]; then
    echo "Error: Game executable not found at $EXECUTABLE"
    echo "Please run setup.sh first to compile the game."
    exit 1
fi

# Create logs directory if it doesn't exist
mkdir -p "$HOME/.cyberhack/logs"

# Launch the game
echo "Launching CYBERHACK..."
"$EXECUTABLE" "$@"
EOF
    
    chmod +x cyberhack
    success_msg "Launcher script created"
}

# Set up configuration files
setup_config() {
    info_msg "Setting up configuration files..."
    
    # Create default config if it doesn't exist
    if [[ ! -f "$HOME/.cyberhack/configs/settings.toml" ]]; then
        cat > "$HOME/.cyberhack/configs/settings.toml" << 'EOF'
[display]
terminal_width = 120
terminal_height = 40
typing_speed = 50
enable_sound = true
fullscreen = false

[gameplay]
difficulty = "normal"
auto_save = true
save_interval = 300
enable_hints = true

[accessibility]
high_contrast = false
large_text = false
screen_reader_support = false

[advanced]
debug_mode = false
developer_console = false
log_level = "info"
EOF
        success_msg "Default configuration created"
    else
        info_msg "Configuration file already exists"
    fi
}

# Final setup steps
final_setup() {
    echo -e "${CYAN}"
    echo "╔════════════════════════════════════════════════════════════════╗"
    echo "║                    INSTALLATION COMPLETE                      ║"
    echo "║                                                                ║"
    echo "║  CYBERHACK has been successfully installed!                   ║"
    echo "║                                                                ║"
    echo "║  To start the game, run:                                      ║"
    echo "║    ./cyberhack                                                 ║"
    echo "║                                                                ║"
    echo "║  Configuration files are located at:                          ║"
    echo "║    ~/.cyberhack/configs/                                       ║"
    echo "║                                                                ║"
    echo "║  Save files will be stored at:                                ║"
    echo "║    ~/.cyberhack/saves/                                         ║"
    echo "║                                                                ║"
    echo "║  For troubleshooting, check logs at:                          ║"
    echo "║    ~/.cyberhack/logs/                                          ║"
    echo "╚════════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
    
    echo -e "${GREEN}"
    echo "    Welcome to the underground, hacker."
    echo "    Your journey into the digital realm begins now..."
    echo -e "${NC}"
}

# Install additional tools (optional)
install_tools() {
    echo -e "${YELLOW}"
    echo "╔════════════════════════════════════════════════════════════════╗"
    echo "║                   OPTIONAL TOOLS INSTALLATION                 ║"
    echo "║                                                                ║"
    echo "║  Would you like to install additional development tools?      ║"
    echo "║  This includes:                                                ║"
    echo "║  - Rust analyzer (IDE support)                                ║"
    echo "║  - Additional Rust components                                  ║"
    echo "║  - Performance monitoring tools                                ║"
    echo "║                                                                ║"
    echo "║  Install additional tools? [y/N]                              ║"
    echo "╚════════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
    
    read -p "Install tools? " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        info_msg "Installing additional tools..."
        
        # Install rust-analyzer if rustup is available
        if command -v rustup &> /dev/null; then
            loading_animation "Installing Rust analyzer"
            rustup component add rust-analyzer --toolchain stable
            success_msg "Rust analyzer installed"
        fi
        
        # Install rustfmt and clippy
        loading_animation "Installing additional Rust components"
        rustup component add rustfmt clippy
        success_msg "Additional components installed"
    fi
}

# Cleanup function
cleanup() {
    info_msg "Cleaning up temporary files..."
    # Remove any temporary files created during installation
    success_msg "Cleanup completed"
}

# Main installation flow
main() {
    print_banner
    check_root
    check_system
    
    if ! check_rust; then
        install_rust
    fi
    
    check_dependencies
    setup_directories
    build_project
    create_launcher
    setup_config
    install_tools
    cleanup
    final_setup
}

# Error handling
trap 'error_msg "Installation interrupted"; exit 1' INT TERM

# Run main installation
main

echo -e "${GREEN}[✓] Installation completed successfully!${NC}"
echo -e "${CYAN}[i] Run './cyberhack' to start the game${NC}"
echo -e "${PURPLE}[i] Happy hacking! 💀⚡${NC}"
