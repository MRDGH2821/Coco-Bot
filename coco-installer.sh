#!/usr/bin/env sh

# Coco Bot Installer Script
# Automatically detects platform and downloads the appropriate binary

set -e # Exit on any error

# Configuration
REPO_OWNER="MRDGH2821"
REPO_NAME="Coco-Bot"
GITHUB_API_URL="https://api.github.com/repos/${REPO_OWNER}/${REPO_NAME}/releases/latest"
GITHUB_RELEASE_URL="https://github.com/${REPO_OWNER}/${REPO_NAME}/releases/download"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
info() {
    printf "${BLUE}[INFO]${NC} %s\n" "$1"
}

warn() {
    printf "${YELLOW}[WARN]${NC} %s\n" "$1"
}

error() {
    printf "${RED}[ERROR]${NC} %s\n" "$1"
    exit 1
}

success() {
    printf "${GREEN}[SUCCESS]${NC} %s\n" "$1"
}

# Check if required tools are available
check_dependencies() {
    info "Checking dependencies..."

    if ! command -v curl >/dev/null 2>&1; then
        error "curl is required but not installed. Please install curl and try again."
    fi

    # Check for extraction tools based on OS
    if [ "${OS}" = "windows" ]; then
        if ! command -v unzip >/dev/null 2>&1; then
            error "unzip is required for Windows archives but not installed. Please install unzip and try again."
        fi
    else
        if ! command -v tar >/dev/null 2>&1; then
            error "tar is required but not installed. Please install tar and try again."
        fi
    fi

    if command -v sha256sum >/dev/null 2>&1; then
        CHECKSUM_CMD="sha256sum"
    elif command -v shasum >/dev/null 2>&1; then
        CHECKSUM_CMD="shasum -a 256"
    else
        warn "No checksum utility found. Skipping checksum verification."
        CHECKSUM_CMD=""
    fi

    success "All dependencies are available"
}

# Detect the operating system
detect_os() {
    case "$(uname -s)" in
    Linux*)
        OS="linux"
        ;;
    Darwin*)
        OS="darwin"
        warn "macOS is not officially supported. Linux binaries might work with compatibility layers."
        ;;
    CYGWIN* | MINGW* | MSYS*)
        OS="windows"
        ;;
    *)
        error "Unsupported operating system: $(uname -s || true)"
        ;;
    esac

    info "Detected OS: ${OS}"
}

# Detect the architecture
detect_arch() {
    case "$(uname -m)" in
    x86_64 | amd64)
        ARCH="amd64"
        ;;
    aarch64 | arm64)
        ARCH="arm64"
        ;;
    armv7l | armv7)
        ARCH="armv7"
        ;;
    armv6l | armv6)
        if [ "${OS}" = "windows" ]; then
            error "ARMv6 is not supported on Windows"
        else
            ARCH="armv7" # Use armv7 binary for armv6 on Linux
            warn "Using armv7 binary for armv6 architecture"
        fi
        ;;
    *)
        error "Unsupported architecture: $(uname -m || true). Supported: x86_64/amd64, aarch64/arm64, armv7"
        ;;
    esac

    # Check architecture availability for Windows
    if [ "${OS}" = "windows" ] && [ "${ARCH}" = "armv7" ]; then
        error "ARMv7 is not supported on Windows. Supported Windows architectures: amd64, arm64"
    fi

    info "Detected architecture: ${ARCH}"
}

# Detect libc type (GNU vs MUSL)
detect_libc() {
    if [ "${OS}" != "linux" ]; then
        LIBC=""
        return
    fi

    # Check if we're running on a musl-based system
    if ldd --version 2>&1 | grep -i musl >/dev/null; then
        LIBC="musl"
    elif ldd --version 2>&1 | grep -i glibc >/dev/null; then
        LIBC="gnu"
    else
        # Check for musl files in common locations
        MUSL_FOUND=""
        for musl_file in /lib/libc.musl-* /usr/lib/libc.musl-*; do
            if [ -f "${musl_file}" ]; then
                MUSL_FOUND="true"
                break
            fi
        done

        if [ -n "${MUSL_FOUND}" ]; then
            LIBC="musl"
        else
            # Default to GNU libc for most Linux distributions
            LIBC="gnu"
            info "Could not detect libc type, defaulting to GNU libc"
        fi
    fi

    info "Detected libc: ${LIBC}"
}

# Get the latest release version
get_latest_version() {
    info "Fetching latest release information..."

    if ! VERSION=$(curl -s "${GITHUB_API_URL}" | grep '"tag_name":' | sed -E 's/.*"tag_name": "([^"]+)".*/\1/'); then
        error "Failed to fetch release information from GitHub API"
    fi

    if [ -z "${VERSION}" ]; then
        error "Could not determine latest version"
    fi

    info "Latest version: ${VERSION}"
}

# Construct the download filename
construct_filename() {
    if [ "${OS}" = "linux" ]; then
        BINARY_NAME="coco-bot-${OS}-${ARCH}-${LIBC}"
        ARCHIVE_NAME="${BINARY_NAME}.tar.gz"
    elif [ "${OS}" = "windows" ]; then
        BINARY_NAME="coco-bot-${OS}-${ARCH}"
        ARCHIVE_NAME="${BINARY_NAME}.zip"
    else
        error "Only Linux and Windows binaries are currently available"
    fi

    CHECKSUM_NAME="${ARCHIVE_NAME}.sha256"

    info "Target archive: ${ARCHIVE_NAME}"
}

# Download the archive and checksum
download_files() {
    DOWNLOAD_URL="${GITHUB_RELEASE_URL}/${VERSION}/${ARCHIVE_NAME}"
    CHECKSUM_URL="${GITHUB_RELEASE_URL}/${VERSION}/${CHECKSUM_NAME}"

    info "Downloading from: ${DOWNLOAD_URL}"

    # Download the archive
    if ! curl -L -o "${ARCHIVE_NAME}" "${DOWNLOAD_URL}"; then
        error "Failed to download ${ARCHIVE_NAME}"
    fi

    # Download the checksum file if checksum command is available
    if [ -n "${CHECKSUM_CMD}" ]; then
        info "Downloading checksum file..."
        if ! curl -L -o "${CHECKSUM_NAME}" "${CHECKSUM_URL}"; then
            warn "Failed to download checksum file, skipping verification"
            CHECKSUM_CMD=""
        fi
    fi

    success "Downloaded ${ARCHIVE_NAME}"
}

# Verify the checksum
verify_checksum() {
    if [ -z "${CHECKSUM_CMD}" ]; then
        warn "Skipping checksum verification (no checksum utility available)"
        return
    fi

    if [ ! -f "${CHECKSUM_NAME}" ]; then
        warn "Checksum file not available, skipping verification"
        return
    fi

    info "Verifying checksum..."

    # Read expected checksum from file
    EXPECTED_CHECKSUM=$(cut -d' ' -f1 "${CHECKSUM_NAME}")

    # Calculate actual checksum
    ACTUAL_CHECKSUM=$(${CHECKSUM_CMD} "${ARCHIVE_NAME}" | cut -d' ' -f1)

    if [ "${EXPECTED_CHECKSUM}" = "${ACTUAL_CHECKSUM}" ]; then
        success "Checksum verification passed"
    else
        error "Checksum verification failed! Expected: ${EXPECTED_CHECKSUM}, Got: ${ACTUAL_CHECKSUM}"
    fi
}

# Extract the archive
extract_archive() {
    info "Extracting ${ARCHIVE_NAME}..."

    if [ "${OS}" = "windows" ]; then
        if ! unzip -q "${ARCHIVE_NAME}"; then
            error "Failed to extract ${ARCHIVE_NAME}"
        fi

        success "Extracted ${ARCHIVE_NAME}"

        # List extracted contents
        info "Extracted files:"
        unzip -l "${ARCHIVE_NAME}" | tail -n +4 | head -n -2 | awk '{print "  " $4}'
    else
        if ! tar -xzf "${ARCHIVE_NAME}"; then
            error "Failed to extract ${ARCHIVE_NAME}"
        fi

        success "Extracted ${ARCHIVE_NAME}"

        # List extracted contents
        info "Extracted files:"
        tar -tzf "${ARCHIVE_NAME}" | sed 's/^/  /'
    fi
}

# Clean up downloaded files
cleanup() {
    if [ "$1" = "keep-archive" ]; then
        info "Keeping archive file: ${ARCHIVE_NAME}"
    else
        info "Cleaning up downloaded files..."
        rm -f "${ARCHIVE_NAME}"
    fi

    rm -f "${CHECKSUM_NAME}"
}

# Make binary executable and show usage info
finalize_installation() {
    # Find the extracted binary
    if [ "${OS}" = "windows" ]; then
        BINARY_PATH=$(find . -name "coco-bot.exe" -type f 2>/dev/null | head -1)
        if [ -z "${BINARY_PATH}" ]; then
            BINARY_PATH=$(find . -name "coco-bot" -type f 2>/dev/null | head -1)
        fi
    else
        # Look for Linux binary (coco-bot.bin)
        BINARY_PATH=$(find . -name "coco-bot.bin" -type f -executable 2>/dev/null | head -1)

        if [ -z "${BINARY_PATH}" ]; then
            # Fallback: look for any coco-bot.bin file and make it executable
            BINARY_PATH=$(find . -name "coco-bot.bin" -type f 2>/dev/null | head -1)
            if [ -n "${BINARY_PATH}" ]; then
                chmod +x "${BINARY_PATH}"
            fi
        fi

        # Final fallback: look for coco-bot without extension
        if [ -z "${BINARY_PATH}" ]; then
            BINARY_PATH=$(find . -name "coco-bot" -type f -executable 2>/dev/null | head -1)
            if [ -z "${BINARY_PATH}" ]; then
                BINARY_PATH=$(find . -name "coco-bot" -type f 2>/dev/null | head -1)
                if [ -n "${BINARY_PATH}" ]; then
                    chmod +x "${BINARY_PATH}"
                fi
            fi
        fi
    fi

    if [ -n "${BINARY_PATH}" ]; then
        success "Coco Bot binary installed at: ${BINARY_PATH}"

        echo ""
        info "Installation complete! You can now run:"
        if [ "${OS}" = "windows" ]; then
            echo "  ${BINARY_PATH}"
        else
            echo "  ./${BINARY_PATH}"
        fi
        echo ""
        info "Make sure to create a .env file with your bot token before running."
        info "See the README.md file for configuration instructions."
    else
        warn "Could not locate the binary after extraction"
        info "Please check the extracted files manually"
    fi
}

# Print usage information
usage() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  -v, --version VERSION    Download specific version (default: latest)"
    echo "  -k, --keep-archive       Keep the downloaded archive file"
    echo "  -p, --platform PLATFORM Override platform detection (linux, windows, darwin)"
    echo "  -a, --arch ARCH          Override architecture detection (amd64, arm64, armv7)"
    echo "  -l, --libc LIBC          Override libc detection for Linux (gnu, musl)"
    echo "  -d, --dry-run            Show what would be downloaded without actually downloading"
    echo "  -h, --help              Show this help message"
    echo ""
    echo "This script automatically detects your platform and downloads the"
    echo "appropriate Coco Bot binary from GitHub releases."
    echo ""
    echo "Manual override examples:"
    echo "  $0 --platform linux --arch amd64 --libc musl"
    echo "  $0 --platform windows --arch arm64"
    echo "  $0 --dry-run --version v0.7.0"
}

# Parse command line arguments
KEEP_ARCHIVE=""
CUSTOM_VERSION=""
OVERRIDE_OS=""
OVERRIDE_ARCH=""
OVERRIDE_LIBC=""
DRY_RUN=""

while [ $# -gt 0 ]; do
    case $1 in
    -v | --version)
        CUSTOM_VERSION="$2"
        shift 2
        ;;
    -k | --keep-archive)
        KEEP_ARCHIVE="keep-archive"
        shift
        ;;
    -p | --platform)
        OVERRIDE_OS="$2"
        shift 2
        ;;
    -a | --arch)
        OVERRIDE_ARCH="$2"
        shift 2
        ;;
    -l | --libc)
        OVERRIDE_LIBC="$2"
        shift 2
        ;;
    -d | --dry-run)
        DRY_RUN="true"
        shift
        ;;
    -h | --help)
        usage
        exit 0
        ;;
    *)
        error "Unknown option: $1"
        ;;
    esac
done

# Validate manual overrides
validate_overrides() {
    if [ -n "${OVERRIDE_OS}" ]; then
        case "${OVERRIDE_OS}" in
        linux | windows | darwin)
            OS="${OVERRIDE_OS}"
            info "Using manual OS override: ${OS}"
            ;;
        *)
            error "Invalid platform: ${OVERRIDE_OS}. Supported: linux, windows, darwin"
            ;;
        esac
    fi

    if [ -n "${OVERRIDE_ARCH}" ]; then
        case "${OVERRIDE_ARCH}" in
        amd64 | arm64 | armv7)
            ARCH="${OVERRIDE_ARCH}"
            info "Using manual architecture override: ${ARCH}"
            ;;
        *)
            error "Invalid architecture: ${OVERRIDE_ARCH}. Supported: amd64, arm64, armv7"
            ;;
        esac
    fi

    if [ -n "${OVERRIDE_LIBC}" ]; then
        case "${OVERRIDE_LIBC}" in
        gnu | musl)
            LIBC="${OVERRIDE_LIBC}"
            info "Using manual libc override: ${LIBC}"
            ;;
        *)
            error "Invalid libc: ${OVERRIDE_LIBC}. Supported: gnu, musl"
            ;;
        esac
    fi

    # Validate platform/architecture compatibility
    if [ -n "${OVERRIDE_OS}" ] && [ -n "${OVERRIDE_ARCH}" ]; then
        if [ "${OS}" = "windows" ] && [ "${ARCH}" = "armv7" ]; then
            error "ARMv7 is not supported on Windows. Supported Windows architectures: amd64, arm64"
        fi
    fi

    # Warn if libc is specified for non-Linux platforms
    if [ -n "${OVERRIDE_LIBC}" ] && [ "${OS}" != "linux" ]; then
        warn "libc setting ignored for non-Linux platform: ${OS}"
        LIBC=""
    fi
}

# Display dry run information
display_dry_run_info() {
    echo ""
    info "=== DRY RUN MODE ==="
    echo ""

    info "Platform Detection Results:"
    echo "  Operating System: ${OS}"
    echo "  Architecture: ${ARCH}"
    if [ "${OS}" = "linux" ]; then
        echo "  Libc Type: ${LIBC}"
    fi
    echo ""

    info "Download Information:"
    echo "  Version: ${VERSION}"
    echo "  Archive: ${ARCHIVE_NAME}"
    echo "  Checksum: ${CHECKSUM_NAME}"
    echo "  Download URL: ${GITHUB_RELEASE_URL}/${VERSION}/${ARCHIVE_NAME}"
    echo "  Checksum URL: ${GITHUB_RELEASE_URL}/${VERSION}/${CHECKSUM_NAME}"
    echo ""

    info "Manual Overrides Applied:"
    if [ -n "${OVERRIDE_OS}" ]; then
        echo "  Platform: ${OVERRIDE_OS} (overridden)"
    else
        echo "  Platform: auto-detected"
    fi

    if [ -n "${OVERRIDE_ARCH}" ]; then
        echo "  Architecture: ${OVERRIDE_ARCH} (overridden)"
    else
        echo "  Architecture: auto-detected"
    fi

    if [ -n "${OVERRIDE_LIBC}" ]; then
        echo "  Libc: ${OVERRIDE_LIBC} (overridden)"
    elif [ "${OS}" = "linux" ]; then
        echo "  Libc: auto-detected"
    else
        echo "  Libc: not applicable"
    fi
    echo ""

    info "Expected Binary:"
    if [ "${OS}" = "windows" ]; then
        echo "  Binary Name: coco-bot.exe or coco-bot"
    else
        echo "  Binary Name: coco-bot.bin"
    fi
    echo ""

    success "Dry run completed. Use without --dry-run to perform actual installation."
}

# Main execution
main() {
    info "Starting Coco Bot installation..."

    # First detect the platform if no overrides are provided
    if [ -z "${OVERRIDE_OS}" ]; then
        detect_os
    fi

    # Validate and apply any manual overrides
    validate_overrides

    check_dependencies

    # Only run auto-detection for values not manually overridden
    if [ -z "${OVERRIDE_OS}" ]; then
        detect_os
    fi

    if [ -z "${OVERRIDE_ARCH}" ]; then
        detect_arch
    fi

    if [ -z "${OVERRIDE_LIBC}" ] && [ "${OS}" = "linux" ]; then
        detect_libc
    fi

    if [ -n "${CUSTOM_VERSION}" ]; then
        VERSION="${CUSTOM_VERSION}"
        info "Using specified version: ${VERSION}"
    else
        get_latest_version
    fi

    construct_filename

    # If dry run mode, show information and exit
    if [ -n "${DRY_RUN}" ]; then
        display_dry_run_info
        exit 0
    fi

    download_files
    verify_checksum
    extract_archive
    cleanup "${KEEP_ARCHIVE}"
    finalize_installation

    success "Coco Bot installation completed successfully!"
}

# Run the main function
main
