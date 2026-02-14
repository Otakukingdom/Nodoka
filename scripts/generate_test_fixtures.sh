#!/bin/bash
# Generate test fixtures for acceptance tests

set -e

FIXTURES_DIR="tests/fixtures"
mkdir -p "$FIXTURES_DIR/audio"
mkdir -p "$FIXTURES_DIR/archives"
mkdir -p "$FIXTURES_DIR/images"

echo "Generating test fixtures..."

# Generate 1-second silent audio files using FFmpeg if available
if command -v ffmpeg &> /dev/null; then
    echo "FFmpeg found, generating real audio fixtures..."
    
    # MP3 - 1 second silent audio
    ffmpeg -f lavfi -i anullsrc=r=44100:cl=mono -t 1 -q:a 9 -acodec libmp3lame \
        "$FIXTURES_DIR/audio/sample_mp3.mp3" -y 2>/dev/null
    
    # M4B - 1 second silent audio  
    ffmpeg -f lavfi -i anullsrc=r=44100:cl=mono -t 1 -c:a aac \
        "$FIXTURES_DIR/audio/sample_m4b.m4b" -y 2>/dev/null
    
    # FLAC - 1 second silent audio
    ffmpeg -f lavfi -i anullsrc=r=44100:cl=mono -t 1 -acodec flac \
        "$FIXTURES_DIR/audio/sample_flac.flac" -y 2>/dev/null
    
    # OGG - 1 second silent audio
    ffmpeg -f lavfi -i anullsrc=r=44100:cl=mono -t 1 -acodec libvorbis \
        "$FIXTURES_DIR/audio/sample_ogg.ogg" -y 2>/dev/null

    # M4A - 1 second silent audio
    ffmpeg -f lavfi -i anullsrc=r=44100:cl=mono -t 1 -c:a aac \
        "$FIXTURES_DIR/audio/sample_m4a.m4a" -y 2>/dev/null

    # OPUS - 1 second silent audio
    ffmpeg -f lavfi -i anullsrc=r=44100:cl=mono -t 1 -acodec libopus \
        "$FIXTURES_DIR/audio/sample_opus.opus" -y 2>/dev/null

    # WAV - 1 second silent audio
    ffmpeg -f lavfi -i anullsrc=r=44100:cl=mono -t 1 -acodec pcm_s16le \
        "$FIXTURES_DIR/audio/sample_wav.wav" -y 2>/dev/null

    # WMA - 1 second silent audio (if supported)
    ffmpeg -f lavfi -i anullsrc=r=44100:cl=mono -t 1 -acodec wmav2 \
        "$FIXTURES_DIR/audio/sample_wma.wma" -y 2>/dev/null || echo "WMA encoding not supported, skipping"

    # AAC - 1 second silent audio
    ffmpeg -f lavfi -i anullsrc=r=44100:cl=mono -t 1 -c:a aac \
        "$FIXTURES_DIR/audio/sample_aac.aac" -y 2>/dev/null

    # M4B with metadata
    ffmpeg -f lavfi -i anullsrc=r=44100:cl=mono -t 1 -c:a aac \
        -metadata title="Test Audiobook" -metadata artist="Test Author" \
        "$FIXTURES_DIR/audio/with_cover.m4b" -y 2>/dev/null

    echo "Audio fixtures generated successfully"
else
    echo "Warning: ffmpeg not found. Creating placeholder files."
    echo "Tests requiring real audio files will be skipped."
    
    # Create minimal placeholder files
    echo "PLACEHOLDER_MP3" > "$FIXTURES_DIR/audio/sample_mp3.mp3"
    echo "PLACEHOLDER_M4B" > "$FIXTURES_DIR/audio/sample_m4b.m4b"
    echo "PLACEHOLDER_M4A" > "$FIXTURES_DIR/audio/sample_m4a.m4a"
    echo "PLACEHOLDER_FLAC" > "$FIXTURES_DIR/audio/sample_flac.flac"
    echo "PLACEHOLDER_OGG" > "$FIXTURES_DIR/audio/sample_ogg.ogg"
    echo "PLACEHOLDER_OPUS" > "$FIXTURES_DIR/audio/sample_opus.opus"
    echo "PLACEHOLDER_WAV" > "$FIXTURES_DIR/audio/sample_wav.wav"
    echo "PLACEHOLDER_WMA" > "$FIXTURES_DIR/audio/sample_wma.wma"
    echo "PLACEHOLDER_AAC" > "$FIXTURES_DIR/audio/sample_aac.aac"
    echo "PLACEHOLDER_M4B_COVER" > "$FIXTURES_DIR/audio/with_cover.m4b"
fi

# Create corrupted file for error handling tests
echo "invalid audio data" > "$FIXTURES_DIR/audio/corrupted.mp3"

# Create sample images (1x1 pixel placeholders)
if command -v convert &> /dev/null; then
    # ImageMagick available - create real images
    convert -size 1x1 xc:blue "$FIXTURES_DIR/images/cover.jpg"
    convert -size 1x1 xc:green "$FIXTURES_DIR/images/folder.png"
else
    # Create placeholder image files
    echo "PLACEHOLDER_JPG" > "$FIXTURES_DIR/images/cover.jpg"
    echo "PLACEHOLDER_PNG" > "$FIXTURES_DIR/images/folder.png"
fi

# Create ZIP archives
if [ -f "$FIXTURES_DIR/audio/sample_mp3.mp3" ]; then
    cd "$FIXTURES_DIR/archives"
    
    # Valid audiobook ZIP
    mkdir -p temp_audiobook
    cp ../audio/sample_mp3.mp3 temp_audiobook/chapter1.mp3
    cp ../audio/sample_mp3.mp3 temp_audiobook/chapter2.mp3
    if command -v zip &> /dev/null; then
        zip -q -r valid_audiobook.zip temp_audiobook
    fi
    rm -rf temp_audiobook
    
    # Corrupted ZIP
    echo "PK corrupted zip data" > corrupted.zip
    
    # Nested ZIP
    mkdir -p nested/subdir
    cp ../audio/sample_mp3.mp3 nested/subdir/audio.mp3
    if command -v zip &> /dev/null; then
        zip -q -r nested.zip nested
    fi
    rm -rf nested
    
    cd - > /dev/null
fi

echo "Fixtures generated in $FIXTURES_DIR"
echo ""
echo "File summary:"
ls -lh "$FIXTURES_DIR/audio/" 2>/dev/null || true
ls -lh "$FIXTURES_DIR/archives/" 2>/dev/null || true
ls -lh "$FIXTURES_DIR/images/" 2>/dev/null || true
