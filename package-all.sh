#!/bin/bash

# === 設定區 ===
APP_NAME="stingtaocreatedesktop"
APP_VERSION="0.1.0"
APP_PATH="src-tauri/target/release/bundle/macos/${APP_NAME}.app"
ZIP_NAME="${APP_NAME}-macos-v${APP_VERSION}.zip"

# === 確認 build 完成 ===
echo "🔨 Building app..."
npm run tauri build

if [ ! -d "$APP_PATH" ]; then
  echo "❌ 找不到 app：$APP_PATH"
  exit 1
fi

# === 移除 quarantine ===
echo "🧹 移除 Gatekeeper 標籤..."
find "$APP_PATH" -print0 | xargs -0 xattr -d com.apple.quarantine 2>/dev/null

# === 本地 adhoc 簽章 ===
echo "🔏 執行本地 codesign（adhoc）..."
codesign --force --deep --sign - "$APP_PATH"

# === 驗證簽章結果 ===
codesign --verify --deep --strict --verbose=2 "$APP_PATH"
if [ $? -ne 0 ]; then
  echo "❌ 簽章失敗，請檢查錯誤訊息"
  exit 1
fi

# === 壓縮 zip（使用 ditto 保留 metadata）===
echo "📦 使用 ditto 建立 zip..."
ditto -c -k --keepParent "$APP_PATH" "$ZIP_NAME"

# === 完成訊息 ===
echo "✅ 完成！產出 zip 檔案：$(pwd)/$ZIP_NAME"
echo "📝 提醒：這個 zip 解壓後應可右鍵開啟 app，不會跳出『損壞』或『無法驗證來源』的錯誤"
