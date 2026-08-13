#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")" && pwd)"

if [ ! -f "$ROOT_DIR/dist/index.html" ]; then
  echo "dist/index.html not found, creating a minimal frontend..."
  mkdir -p "$ROOT_DIR/dist"
  cat > "$ROOT_DIR/dist/index.html" <<'EOF'
<!doctype html>
<html lang="zh-CN">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>cmd-down</title>
  </head>
  <body>
    <p>cmd-down is running in menu bar.</p>
  </body>
</html>
EOF
fi

cd "$ROOT_DIR/src-tauri"

if ! cargo tauri --version >/dev/null 2>&1; then
  echo "Installing tauri-cli..."
  cargo install tauri-cli --locked
fi

echo "Building macOS app bundle with Tauri..."
cargo tauri build

echo "Build finished. Artifacts are under src-tauri/target/release/bundle/."
