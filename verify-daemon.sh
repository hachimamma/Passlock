#!/bin/bash

echo "PassLock Daemon Installation Check"
echo "======================================"
echo ""

ERRORS=0
WARNINGS=0

if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    OS="linux"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    OS="macos"
else
    echo "Unsupported OS: $OSTYPE"
    exit 1
fi

echo "OS: $OS"
echo ""

echo "Checking PassLock binary..."
if command -v passlock &> /dev/null; then
    PASSLOCK_PATH=$(which passlock)
    echo "   Found: $PASSLOCK_PATH"
    
    VERSION=$(passlock version 2>/dev/null || echo "unknown")
    echo "   Version: $VERSION"
else
    echo "   PassLock binary not found!"
    echo "      Install with: ./install-daemon.sh"
    ERRORS=$((ERRORS + 1))
fi
echo ""

echo "Checking vault..."
if [ -f ~/.passlock.vault ]; then
    SIZE=$(du -h ~/.passlock.vault | cut -f1)
    echo "   Vault exists (~/.passlock.vault)"
    echo "      Size: $SIZE"
else
    echo "   No vault found"
    echo "      Create with: passlock create <password>"
    WARNINGS=$((WARNINGS + 1))
fi
echo ""

if [ "$OS" = "linux" ]; then
    echo "Checking Linux dependencies..."
    
    if [ -n "$WAYLAND_DISPLAY" ]; then
        echo "   Display: Wayland"
        
        if command -v ydotool &> /dev/null; then
            echo "   ydotool found"
            
            if systemctl is-active --quiet ydotoold 2>/dev/null; then
                echo "   ydotoold service running"
            else
                echo "   ydotoold service not running"
                echo "      Start with: sudo systemctl start ydotoold"
                WARNINGS=$((WARNINGS + 1))
            fi
        else
            echo "   ydotool not found!"
            echo "      Install: sudo apt install ydotool"
            ERRORS=$((ERRORS + 1))
        fi
    else
        echo "   Display: X11"
        
        if command -v xdotool &> /dev/null; then
            echo "   xdotool found"
        else
            echo "   xdotool not found!"
            echo "      Install: sudo apt install xdotool"
            ERRORS=$((ERRORS + 1))
        fi
    fi
    echo ""
    
    echo "Checking systemd service..."
    if [ -f ~/.config/systemd/user/passlock.service ]; then
        echo "   Service file exists"
        
        if systemctl --user is-enabled passlock &> /dev/null; then
            echo "   Service enabled"
        else
            echo "   Service not enabled"
            echo "      Enable: systemctl --user enable passlock"
            WARNINGS=$((WARNINGS + 1))
        fi
        
        if systemctl --user is-active passlock &> /dev/null; then
            echo "   Service running"
        else
            echo "   Service not running (start manually)"
        fi
    else
        echo "   Service file not found"
        echo "      Install with: ./install-daemon.sh"
        WARNINGS=$((WARNINGS + 1))
    fi
    echo ""
fi

if [ "$OS" = "macos" ]; then
    echo "Checking macOS dependencies..."
    echo "   No external dependencies needed!"
    echo ""
    
    echo "Checking LaunchAgent..."
    PLIST="$HOME/Library/LaunchAgents/com.passlock.daemon.plist"
    if [ -f "$PLIST" ]; then
        echo "   LaunchAgent exists"
        
        if launchctl list | grep -q com.passlock.daemon; then
            echo "   LaunchAgent loaded"
        else
            echo "   LaunchAgent not loaded"
            echo "      Load: launchctl load $PLIST"
            WARNINGS=$((WARNINGS + 1))
        fi
    else
        echo "   LaunchAgent not found"
        echo "      Install with: ./install-daemon.sh"
        WARNINGS=$((WARNINGS + 1))
    fi
    echo ""
fi

echo "Checking Rust environment..."
if command -v cargo &> /dev/null; then
    RUST_VERSION=$(rustc --version | cut -d' ' -f2)
    echo "Rust installed (v$RUST_VERSION)"
else
    echo "Rust not found (only needed for building)"
fi
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━"
echo "SUMMARY"
echo "━━━━━━━━━━━━━━━━━━━━━━━━"
if [ $ERRORS -eq 0 ] && [ $WARNINGS -eq 0 ]; then
    echo "All checks passed"
    echo ""
    echo "Passlock daemon setup complete"
    echo ""
    echo "Quick start:"
    echo "  passlock daemon start"
    echo ""
    echo "Hotkeys:"
    echo "  Ctrl+Shift+P - Capture password"
    echo "  Ctrl+Shift+A - Auto-fill"
    echo "  Ctrl+Shift+L - Lock vault"
elif [ $ERRORS -gt 0 ]; then
    echo "$ERRORS error(s) found"
    echo "$WARNINGS warning(s) found"
    echo ""
    echo "Please fix the errors above before using the daemon."
else
    echo "$WARNINGS warning(s) found"
    echo ""
    echo "The daemon should work, but some features may be limited."
fi
echo ""

exit $ERRORS