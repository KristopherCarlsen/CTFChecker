#!/bin/sh

# Exit on any failure.
set -euo pipefail

# Check for root priviliages.
if [[ "$EUID" -ne 0 ]]; then
    echo "Root privileges are required to run the installer. Try running it with sudo."
    exit 1
fi

SRC_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
INSTALL_DIR="/srv/ctfchecker"
SERVICE_FILE="ctfchecker.service"

# Copy the install files to thier respective locations.
declare -A FILES=(
    ["check.sh"]="$INSTALL_DIR/check.sh"
    ["ctfchecker.conf"]="$INSTALL_DIR/ctfchecker.conf"
    ["target/release/ctfchecker"]="$INSTALL_DIR/ctfchecker"
    ["ctfchecker.service"]="/etc/systemd/system/$SERVICE_FILE"
)

install_file(){
    local src="$SRC_DIR/$1"
    local dst="$2"
    local dst_dir="$(dirname "$dst")"
    if [[ ! -d dst_dir ]]; then
        echo "Creating directory $dst_dir"
        mkdir -p "$dst_dir"
    fi

    echo "Installing file $src to $dst"
    cp "$src" "$dst"
}

echo "Starting Install"

# Copy over the needed files.
for src in "${!FILES[@]}"; do
    install_file "$src" "${FILES[$src]}"
done

# Reload the systemd daemons and enable our new one.
echo "Enabling systemd service."
systemctl daemon-reload
systemctl enable $SERVICE_FILE
systemctl start $SERVICE_FILE

echo "Install Complete"