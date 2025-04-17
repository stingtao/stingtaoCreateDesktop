#!/bin/bash

# === 基本設定 ===
APP_NAME="stingtaocreatedesktop"
APP_VERSION="0.1.0"

# === 前處理 ===
echo "🧹 清除舊 build..."
rm -rf src-tauri/target/release/bundle

# === 執行 Tauri 打包 ===
echo "🔨 Building app for all platforms (current OS only)..."
npm run tauri build

# === macOS 專用處理 ===
if [[ "$OSTYPE" == "darwin"* ]]; then
  MACOS_APP_PATH="src-tauri/target/release/bundle/macos/${APP_NAME}.app"
  MACOS_ZIP_NAME="${APP_NAME}-macos-v${APP_VERSION}.zip"

  if [ -d "$MACOS_APP_PATH" ]; then
    echo "🧹 Removing quarantine flags..."
    find "$MACOS_APP_PATH" -exec xattr -d com.apple.quarantine {} \; 2>/dev/null

    echo "📦 Zipping macOS app..."
    cd "$(dirname "$MACOS_APP_PATH")"
    zip -r "../../../../../${MACOS_ZIP_NAME}" "$(basename "$MACOS_APP_PATH")"
    cd - > /dev/null
    echo "✅ macOS zip created at: ${MACOS_ZIP_NAME}"
  else
    echo "⚠️ macOS app not found."
  fi
fi

# === Windows 專用處理（需在 Windows 環境執行） ===
if [[ "$OSTYPE" == "msys"* || "$OSTYPE" == "win32" ]]; then
  WINDOWS_EXE_PATH="src-tauri/target/release/bundle/windows/${APP_NAME}.exe"
  WINDOWS_MSI_PATH="src-tauri/target/release/bundle/windows/${APP_NAME}_${APP_VERSION}_x64_en-US.msi"
  WINDOWS_ZIP_NAME="${APP_NAME}-windows-v${APP_VERSION}.zip"

  if [ -f "$WINDOWS_EXE_PATH" ]; then
    echo "📦 Zipping Windows executable..."
    zip -j "${WINDOWS_ZIP_NAME}" "$WINDOWS_EXE_PATH" "$WINDOWS_MSI_PATH"
    echo "✅ Windows zip created at: ${WINDOWS_ZIP_NAME}"
  else
    echo "⚠️ Windows exe not found."
  fi
fi

echo "🎉 All done!"
