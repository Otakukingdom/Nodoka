#!/bin/bash
# macOS DMG creation script for Nodoka Audiobook Reader

set -e

APP_NAME="Nodoka Audiobook Reader"
VERSION="0.2.0"
BUNDLE_NAME="Nodoka.app"
DMG_NAME="Nodoka-${VERSION}.dmg"
BUILD_DIR="../../target/release"
BUNDLE_DIR="${BUILD_DIR}/${BUNDLE_NAME}"
TEMP_DMG="temp.dmg"
DEFAULT_BINARY="${BUILD_DIR}/nodoka"
UNIVERSAL_BINARY="${BUILD_DIR}/nodoka-universal"

echo "Creating macOS application bundle..."

# Create application bundle structure
mkdir -p "${BUNDLE_DIR}/Contents/MacOS"
mkdir -p "${BUNDLE_DIR}/Contents/Resources"

# Copy binary
if [ -n "${NODOKA_BINARY:-}" ]; then
    BINARY_PATH="${NODOKA_BINARY}"
elif [ -f "${UNIVERSAL_BINARY}" ]; then
    BINARY_PATH="${UNIVERSAL_BINARY}"
else
    BINARY_PATH="${DEFAULT_BINARY}"
fi

if [ ! -f "${BINARY_PATH}" ]; then
    echo "Error: Nodoka binary not found at ${BINARY_PATH}"
    exit 1
fi

cp "${BINARY_PATH}" "${BUNDLE_DIR}/Contents/MacOS/nodoka"
chmod +x "${BUNDLE_DIR}/Contents/MacOS/nodoka"

# Copy icon
ICON_SOURCE_512="../../assets/icons/Entypo_d83d(0)_512.png"
ICON_SOURCE_256="../../assets/icons/Entypo_d83d(0)_256.png"

if command -v iconutil >/dev/null 2>&1 && command -v sips >/dev/null 2>&1; then
    if [ -f "${ICON_SOURCE_512}" ]; then
        ICON_SOURCE="${ICON_SOURCE_512}"
    elif [ -f "${ICON_SOURCE_256}" ]; then
        ICON_SOURCE="${ICON_SOURCE_256}"
    else
        ICON_SOURCE=""
    fi

    if [ -n "${ICON_SOURCE}" ]; then
        # Convert PNG to ICNS with a complete iconset
        mkdir -p AppIcon.iconset
        sips -z 16 16 "${ICON_SOURCE}" --out AppIcon.iconset/icon_16x16.png >/dev/null
        sips -z 32 32 "${ICON_SOURCE}" --out AppIcon.iconset/icon_16x16@2x.png >/dev/null
        sips -z 32 32 "${ICON_SOURCE}" --out AppIcon.iconset/icon_32x32.png >/dev/null
        sips -z 64 64 "${ICON_SOURCE}" --out AppIcon.iconset/icon_32x32@2x.png >/dev/null
        sips -z 128 128 "${ICON_SOURCE}" --out AppIcon.iconset/icon_128x128.png >/dev/null
        sips -z 256 256 "${ICON_SOURCE}" --out AppIcon.iconset/icon_128x128@2x.png >/dev/null
        sips -z 256 256 "${ICON_SOURCE}" --out AppIcon.iconset/icon_256x256.png >/dev/null
        sips -z 512 512 "${ICON_SOURCE}" --out AppIcon.iconset/icon_256x256@2x.png >/dev/null
        sips -z 512 512 "${ICON_SOURCE}" --out AppIcon.iconset/icon_512x512.png >/dev/null
        sips -z 1024 1024 "${ICON_SOURCE}" --out AppIcon.iconset/icon_512x512@2x.png >/dev/null
        iconutil -c icns AppIcon.iconset -o "${BUNDLE_DIR}/Contents/Resources/AppIcon.icns"
        rm -rf AppIcon.iconset
    else
        echo "Icon source not found; skipping icon generation."
    fi
else
    echo "iconutil or sips not available; skipping icon generation."
fi

# Create Info.plist
cat > "${BUNDLE_DIR}/Contents/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>English</string>
    <key>CFBundleExecutable</key>
    <string>nodoka</string>
    <key>CFBundleIconFile</key>
    <string>AppIcon</string>
    <key>CFBundleIdentifier</key>
    <string>com.otakukingdom.nodoka</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>${APP_NAME}</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>${VERSION}</string>
    <key>CFBundleVersion</key>
    <string>${VERSION}</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.15</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>CFBundleDocumentTypes</key>
    <array>
        <dict>
            <key>CFBundleTypeExtensions</key>
            <array>
                <string>mp3</string>
                <string>m4a</string>
                <string>m4b</string>
                <string>ogg</string>
                <string>flac</string>
                <string>opus</string>
            </array>
            <key>CFBundleTypeName</key>
            <string>Audio File</string>
            <key>CFBundleTypeRole</key>
            <string>Viewer</string>
        </dict>
    </array>
</dict>
</plist>
EOF

echo "Creating DMG..."

# Create temporary DMG
hdiutil create -size 100m -fs HFS+ -volname "${APP_NAME}" "${TEMP_DMG}"

# Mount the DMG
hdiutil attach "${TEMP_DMG}" -mountpoint /Volumes/"${APP_NAME}"

# Copy app bundle to DMG
cp -R "${BUNDLE_DIR}" /Volumes/"${APP_NAME}/"

# Create symlink to Applications folder
ln -s /Applications /Volumes/"${APP_NAME}/Applications"

# Unmount
hdiutil detach /Volumes/"${APP_NAME}"

# Convert to compressed DMG
hdiutil convert "${TEMP_DMG}" -format UDZO -o "${DMG_NAME}"

# Clean up
rm "${TEMP_DMG}"

echo "DMG created: ${DMG_NAME}"
