#!/bin/bash
# PassLock Daemon Installation Script

set -e

echo "🔐 PassLock Daemon Installer"
echo "=============================="
echo ""

# Check OS
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    OS="linux"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    OS="macos"
else
    echo "❌ Unsupported OS: $OSTYPE"
    exit 1
fi

echo "📍 Detected OS: $OS"
echo ""

# Check dependencies
echo "🔍 Checking dependencies..."

if [ "$OS" = "linux" ]; then
    # Check for xdotool or ydotool
    if ! command -v xdotool &> /dev/null && ! command -v ydotool &> /dev/null; then
        echo "⚠️  Neither xdotool nor ydotool found"
        echo "   Install one for auto-fill functionality:"
        echo "   • X11: sudo apt install xdotool"
        echo "   • Wayland: sudo apt install ydotool"
        echo ""
    fi
fi

# Build PassLock
echo "🔨 Building PassLock with daemon support..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "❌ Build failed"
    exit 1
fi

echo "✅ Build successful"
echo ""

# Install binary
echo "📦 Installing binary..."
sudo cp target/release/passlock /usr/local/bin/passlock
sudo chmod +x /usr/local/bin/passlock
echo "✅ Binary installed to /usr/local/bin/passlock"
echo ""

# Install systemd service (Linux only)
if [ "$OS" = "linux" ]; then
    echo "🔧 Installing systemd service..."
    
    mkdir -p ~/.config/systemd/user
    cp systemd/passlock.service ~/.config/systemd/user/
    
    systemctl --user daemon-reload
    systemctl --user enable passlock.service
    
    echo "✅ Systemd service installed"
    echo ""
    
    echo "📋 Service commands:"
    echo "   Start:   systemctl --user start passlock"
    echo "   Stop:    systemctl --user stop passlock"
    echo "   Status:  systemctl --user status passlock"
    echo "   Logs:    journalctl --user -u passlock -f"
    echo ""
fi

# Install LaunchAgent (macOS only)
if [ "$OS" = "macos" ]; then
    echo "🔧 Installing LaunchAgent..."
    
    PLIST_FILE="$HOME/Library/LaunchAgents/com.passlock.daemon.plist"
    
    cat > "$PLIST_FILE" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.passlock.daemon</string>
    <key>ProgramArguments</key>
    <array>
        <string>/usr/local/bin/passlock</string>
        <string>daemon</string>
        <string>start</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>StandardOutPath</key>
    <string>$HOME/Library/Logs/passlock.log</string>
    <key>StandardErrorPath</key>
    <string>$HOME/Library/Logs/passlock-error.log</string>
</dict>
</plist>
EOF
    
    launchctl load "$PLIST_FILE"
    
    echo "✅ LaunchAgent installed"
    echo ""
    
    echo "📋 Service commands:"
    echo "   Start:   launchctl start com.passlock.daemon"
    echo "   Stop:    launchctl stop com.passlock.daemon"
    echo "   Logs:    tail -f ~/Library/Logs/passlock.log"
    echo ""
fi

echo "🎉 Installation complete!"
echo ""
echo "🚀 Quick Start:"
echo "   1. Create vault:  passlock create <password>"
echo "   2. Start daemon:  passlock daemon start"
echo ""
echo "⌨️  Hotkeys:"
echo "   • Ctrl+Shift+P - Capture password"
echo "   • Ctrl+Shift+A - Auto-fill"
echo "   • Ctrl+Shift+L - Lock vault"
echo ""
echo "📚 Documentation: https://github.com/yourusername/passlock"