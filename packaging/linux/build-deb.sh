#!/bin/bash
# Debian package build script for Nodoka Audiobook Reader

set -e

APP_NAME="nodoka"
VERSION="0.2.0"
ARCH="amd64"
PKG_NAME="${APP_NAME}_${VERSION}_${ARCH}"
BUILD_DIR="../../target/release"
PKG_DIR="${PKG_NAME}"

if [ -n "${TARGET_TRIPLE:-}" ]; then
    BUILD_DIR="../../target/${TARGET_TRIPLE}/release"
fi

if [ -n "${TARGET_DIR:-}" ]; then
    BUILD_DIR="${TARGET_DIR}"
fi

echo "Building Debian package for Nodoka Audiobook Reader..."

# Create package directory structure
mkdir -p "${PKG_DIR}/DEBIAN"
mkdir -p "${PKG_DIR}/usr/bin"
mkdir -p "${PKG_DIR}/usr/share/applications"
mkdir -p "${PKG_DIR}/usr/share/icons/hicolor/256x256/apps"
mkdir -p "${PKG_DIR}/usr/share/doc/${APP_NAME}"

# Copy binary
if [ ! -f "${BUILD_DIR}/nodoka" ]; then
    echo "Error: Nodoka binary not found at ${BUILD_DIR}/nodoka"
    echo "Set TARGET_TRIPLE or TARGET_DIR to the release output directory."
    exit 1
fi

cp "${BUILD_DIR}/nodoka" "${PKG_DIR}/usr/bin/"
chmod 755 "${PKG_DIR}/usr/bin/nodoka"

# Copy desktop file
cp nodoka.desktop "${PKG_DIR}/usr/share/applications/"

# Copy icon
cp "../../icons/app/Entypo_d83d(0)_256.png" "${PKG_DIR}/usr/share/icons/hicolor/256x256/apps/nodoka.png"

# Create control file
cat > "${PKG_DIR}/DEBIAN/control" << EOF
Package: nodoka
Version: ${VERSION}
Section: sound
Priority: optional
Architecture: ${ARCH}
Depends: vlc, libvlc5, libvlccore9
Maintainer: Otakukingdom Co <mistlight@otakukingdom.com>
Description: A free and open source cross-platform audiobook reader
 Nodoka is an audiobook player that helps you organize and listen to
 your audiobook collection. It tracks your progress through audiobooks
 and resumes playback where you left off.
 .
 Features:
  - Multi-format support (MP3, M4A, M4B, OGG, FLAC, OPUS)
  - Automatic audiobook discovery
  - Progress tracking and resume playback
  - Playback speed control
  - Clean and intuitive interface
Homepage: https://github.com/otakukingdom/nodoka
EOF

# Create copyright file
cat > "${PKG_DIR}/usr/share/doc/${APP_NAME}/copyright" << EOF
Format: https://www.debian.org/doc/packaging-manuals/copyright-format/1.0/
Upstream-Name: nodoka
Upstream-Contact: Mistlight Oriroris
Source: https://github.com/otakukingdom/nodoka

Files: *
Copyright: 2024 Otakukingdom Co
License: MIT
 Permission is hereby granted, free of charge, to any person obtaining a
 copy of this software and associated documentation files (the "Software"),
 to deal in the Software without restriction, including without limitation
 the rights to use, copy, modify, merge, publish, distribute, sublicense,
 and/or sell copies of the Software, and to permit persons to whom the
 Software is furnished to do so, subject to the following conditions:
 .
 The above copyright notice and this permission notice shall be included
 in all copies or substantial portions of the Software.
 .
 THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
 OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 DEALINGS IN THE SOFTWARE.
EOF

# Create changelog
cat > "${PKG_DIR}/usr/share/doc/${APP_NAME}/changelog.Debian.gz" << EOF
nodoka (${VERSION}) unstable; urgency=medium

  * Initial Rust release
  * Converted from C++/Qt to Rust/iced
  * Improved performance and memory safety

 -- Mistlight Oriroris <mistlight@otakukingdom.com>  $(date -R)
EOF
gzip -9 "${PKG_DIR}/usr/share/doc/${APP_NAME}/changelog.Debian.gz"

# Create postinst script
cat > "${PKG_DIR}/DEBIAN/postinst" << EOF
#!/bin/sh
set -e

# Update desktop database
if [ -x /usr/bin/update-desktop-database ]; then
    update-desktop-database -q /usr/share/applications
fi

# Update icon cache
if [ -x /usr/bin/gtk-update-icon-cache ]; then
    gtk-update-icon-cache -q -t -f /usr/share/icons/hicolor || true
fi

exit 0
EOF
chmod 755 "${PKG_DIR}/DEBIAN/postinst"

# Create postrm script
cat > "${PKG_DIR}/DEBIAN/postrm" << EOF
#!/bin/sh
set -e

if [ "\$1" = "remove" ] || [ "\$1" = "purge" ]; then
    # Update desktop database
    if [ -x /usr/bin/update-desktop-database ]; then
        update-desktop-database -q /usr/share/applications
    fi
    
    # Update icon cache
    if [ -x /usr/bin/gtk-update-icon-cache ]; then
        gtk-update-icon-cache -q -t -f /usr/share/icons/hicolor || true
    fi
fi

exit 0
EOF
chmod 755 "${PKG_DIR}/DEBIAN/postrm"

# Build the package
dpkg-deb --build "${PKG_DIR}"

echo "Debian package created: ${PKG_NAME}.deb"

# Clean up
rm -rf "${PKG_DIR}"
