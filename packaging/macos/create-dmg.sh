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

echo "Creating macOS application bundle..."

# Create application bundle structure
mkdir -p "${BUNDLE_DIR}/Contents/MacOS"
mkdir -p "${BUNDLE_DIR}/Contents/Resources"

# Copy binary
cp "${BUILD_DIR}/nodoka" "${BUNDLE_DIR}/Contents/MacOS/"
chmod +x "${BUNDLE_DIR}/Contents/MacOS/nodoka"

# Copy icon
if [ -f "../../assets/icons/Entypo_d83d(0)_256.png" ]; then
    # Convert PNG to ICNS (requires iconutil on macOS)
    mkdir -p AppIcon.iconset
    cp "../../assets/icons/Entypo_d83d(0)_256.png" AppIcon.iconset/icon_256x256.png
    cp "../../assets/icons/Entypo_d83d(0)_512.png" AppIcon.iconset/icon_512x512.png
    iconutil -c icns AppIcon.iconset -o "${BUNDLE_DIR}/Contents/Resources/AppIcon.icns"
    rm -rf AppIcon.iconset
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
