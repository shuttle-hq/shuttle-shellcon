#!/bin/bash
# Quick Node.js 20 setup for ShellCon DevContainer
echo "🚀 Installing Node.js 20..."
curl -fsSL https://deb.nodesource.com/setup_20.x | bash -
apt-get install -y nodejs
echo "✅ Node.js installed: $(node --version)"
echo "✅ npm installed: $(npm --version)"