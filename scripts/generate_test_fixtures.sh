#!/bin/bash
# Generate test fixtures for acceptance tests

set -e

FIXTURES_DIR="tests/fixtures"
mkdir -p "$FIXTURES_DIR/audio"
mkdir -p "$FIXTURES_DIR/archives"
mkdir -p "$FIXTURES_DIR/images"

echo "Generating test fixtures..."

ensure_dir() {
    mkdir -p "$1"
}

file_missing() {
    [ ! -f "$1" ]
}

# Generate 1-second silent audio files using FFmpeg if available
if command -v ffmpeg &> /dev/null; then
    echo "FFmpeg found, generating real audio fixtures..."
    
    # MP3 - 1 second silent audio
    if file_missing "$FIXTURES_DIR/audio/sample_mp3.mp3"; then
        ffmpeg -n -f lavfi -i anullsrc=r=44100:cl=mono -t 1 -q:a 9 -acodec libmp3lame \
            "$FIXTURES_DIR/audio/sample_mp3.mp3" 2>/dev/null
    fi
    
    # M4B - 1 second silent audio
    if file_missing "$FIXTURES_DIR/audio/sample_m4b.m4b"; then
        ffmpeg -n -f lavfi -i anullsrc=r=44100:cl=mono -t 1 -c:a aac \
            "$FIXTURES_DIR/audio/sample_m4b.m4b" 2>/dev/null
    fi
    
    # FLAC - 1 second silent audio
    if file_missing "$FIXTURES_DIR/audio/sample_flac.flac"; then
        ffmpeg -n -f lavfi -i anullsrc=r=44100:cl=mono -t 1 -acodec flac \
            "$FIXTURES_DIR/audio/sample_flac.flac" 2>/dev/null
    fi
    
    # OGG - 1 second silent audio
    if file_missing "$FIXTURES_DIR/audio/sample_ogg.ogg"; then
        ffmpeg -n -f lavfi -i anullsrc=r=44100:cl=mono -t 1 -acodec libvorbis \
            "$FIXTURES_DIR/audio/sample_ogg.ogg" 2>/dev/null
    fi

    # M4A - 1 second silent audio
    if file_missing "$FIXTURES_DIR/audio/sample_m4a.m4a"; then
        ffmpeg -n -f lavfi -i anullsrc=r=44100:cl=mono -t 1 -c:a aac \
            "$FIXTURES_DIR/audio/sample_m4a.m4a" 2>/dev/null
    fi

    # OPUS - 1 second silent audio
    if file_missing "$FIXTURES_DIR/audio/sample_opus.opus"; then
        ffmpeg -n -f lavfi -i anullsrc=r=44100:cl=mono -t 1 -acodec libopus \
            "$FIXTURES_DIR/audio/sample_opus.opus" 2>/dev/null
    fi

    # WAV - 1 second silent audio
    if file_missing "$FIXTURES_DIR/audio/sample_wav.wav"; then
        ffmpeg -n -f lavfi -i anullsrc=r=44100:cl=mono -t 1 -acodec pcm_s16le \
            "$FIXTURES_DIR/audio/sample_wav.wav" 2>/dev/null
    fi

    # WMA - 1 second silent audio (if supported)
    if file_missing "$FIXTURES_DIR/audio/sample_wma.wma"; then
        ffmpeg -n -f lavfi -i anullsrc=r=44100:cl=mono -t 1 -acodec wmav2 \
            "$FIXTURES_DIR/audio/sample_wma.wma" 2>/dev/null || echo "WMA encoding not supported, skipping"
    fi

    # AAC - 1 second silent audio
    if file_missing "$FIXTURES_DIR/audio/sample_aac.aac"; then
        ffmpeg -n -f lavfi -i anullsrc=r=44100:cl=mono -t 1 -c:a aac \
            "$FIXTURES_DIR/audio/sample_aac.aac" 2>/dev/null
    fi

    # M4B with metadata
    if file_missing "$FIXTURES_DIR/audio/with_cover.m4b"; then
        ffmpeg -n -f lavfi -i anullsrc=r=44100:cl=mono -t 1 -c:a aac \
            -metadata title="Test Audiobook" -metadata artist="Test Author" \
            "$FIXTURES_DIR/audio/with_cover.m4b" 2>/dev/null
    fi

    echo "Audio fixtures generated successfully"
else
    echo "Warning: ffmpeg not found. Creating placeholder files."
    echo "FFmpeg not available; generating valid WAV-based fixtures via Python."

    python3 - <<'PY'
import wave
from pathlib import Path

fixtures_dir = Path('tests/fixtures/audio')
fixtures_dir.mkdir(parents=True, exist_ok=True)

wav_path = fixtures_dir / 'sample_wav.wav'

sample_rate = 44100
duration_seconds = 1
num_frames = sample_rate * duration_seconds

if not wav_path.exists():
    with wave.open(str(wav_path), 'wb') as wf:
        wf.setnchannels(1)
        wf.setsampwidth(2)
        wf.setframerate(sample_rate)
        wf.writeframes(b'\x00\x00' * num_frames)

print(f'Generated {wav_path} ({wav_path.stat().st_size} bytes)')
PY

    # Copy the valid WAV bytes to other extensions so VLC can still play them.
    for dest in \
        "$FIXTURES_DIR/audio/sample_mp3.mp3" \
        "$FIXTURES_DIR/audio/sample_m4b.m4b" \
        "$FIXTURES_DIR/audio/sample_m4a.m4a" \
        "$FIXTURES_DIR/audio/sample_flac.flac" \
        "$FIXTURES_DIR/audio/sample_ogg.ogg" \
        "$FIXTURES_DIR/audio/sample_opus.opus" \
        "$FIXTURES_DIR/audio/sample_wma.wma" \
        "$FIXTURES_DIR/audio/sample_aac.aac" \
        "$FIXTURES_DIR/audio/with_cover.m4b"; do
        if file_missing "$dest"; then
            cp "$FIXTURES_DIR/audio/sample_wav.wav" "$dest"
        fi
    done
fi

# Create corrupted file for error handling tests
if file_missing "$FIXTURES_DIR/audio/corrupted.mp3"; then
    echo "invalid audio data" > "$FIXTURES_DIR/audio/corrupted.mp3"
fi

# Create sample images (1x1 pixel) without overwriting committed fixtures.
if command -v convert &> /dev/null; then
    if file_missing "$FIXTURES_DIR/images/cover.jpg"; then
        convert -size 1x1 xc:blue "$FIXTURES_DIR/images/cover.jpg"
    fi
    if file_missing "$FIXTURES_DIR/images/folder.png"; then
        convert -size 1x1 xc:green "$FIXTURES_DIR/images/folder.png"
    fi
else
    python3 - <<'PY'
from __future__ import annotations

import base64
from pathlib import Path

images_dir = Path('tests/fixtures/images')
images_dir.mkdir(parents=True, exist_ok=True)

# 1x1 PNG (opaque green) and 1x1 JPEG (opaque blue)
png_b64 = (
    'iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAIAAACQd1PeAAAADElEQVR4nGNg+A8AAQUB'
    'AScY42YAAAAASUVORK5CYII='
)
jpeg_b64 = (
    '/9j/4AAQSkZJRgABAQAAAQABAAD/2wCEAAEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEB'
    'AQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAf/'
    '2wCEAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEB'
    'AQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAf/wAARCAAQABADASIAAhEBAxEB'
    '/8QAFQABAQAAAAAAAAAAAAAAAAAAAAb/xAAUEAEAAAAAAAAAAAAAAAAAAAAA/9oADAMBAAIQAxAAAAFf'
    '/8QAFBABAAAAAAAAAAAAAAAAAAAAAP/aAAgBAQABBQJf/8QAFBEBAAAAAAAAAAAAAAAAAAAAAP/aAAgBAwEB'
    'PwF//8QAFBEBAAAAAAAAAAAAAAAAAAAAAP/aAAgBAgEBPwF//8QAFBABAAAAAAAAAAAAAAAAAAAAAP/aAAgBAQ'
    'AGPwJf/8QAFBABAAAAAAAAAAAAAAAAAAAAAP/aAAgBAQABPyFf/9k='
)

cover = images_dir / 'cover.jpg'
folder = images_dir / 'folder.png'

if not cover.exists():
    cover.write_bytes(base64.b64decode(jpeg_b64))

if not folder.exists():
    folder.write_bytes(base64.b64decode(png_b64))
PY
fi

# Create ZIP archives (only if missing)
if [ -f "$FIXTURES_DIR/audio/sample_mp3.mp3" ] && command -v zip &> /dev/null; then
    archives_dir="$FIXTURES_DIR/archives"

    if file_missing "$archives_dir/valid_audiobook.zip"; then
        rm -rf "$archives_dir/temp_audiobook"
        ensure_dir "$archives_dir/temp_audiobook"
        cp "$FIXTURES_DIR/audio/sample_mp3.mp3" "$archives_dir/temp_audiobook/chapter1.mp3"
        cp "$FIXTURES_DIR/audio/sample_mp3.mp3" "$archives_dir/temp_audiobook/chapter2.mp3"
        (cd "$archives_dir" && zip -q -r valid_audiobook.zip temp_audiobook)
        rm -rf "$archives_dir/temp_audiobook"
    fi

    if file_missing "$archives_dir/corrupted.zip"; then
        echo "PK corrupted zip data" > "$archives_dir/corrupted.zip"
    fi

    if file_missing "$archives_dir/nested.zip"; then
        rm -rf "$archives_dir/nested"
        ensure_dir "$archives_dir/nested/subdir"
        cp "$FIXTURES_DIR/audio/sample_mp3.mp3" "$archives_dir/nested/subdir/audio.mp3"
        (cd "$archives_dir" && zip -q -r nested.zip nested)
        rm -rf "$archives_dir/nested"
    fi
fi

echo "Fixtures generated in $FIXTURES_DIR"
echo ""
echo "File summary:"
ls -lh "$FIXTURES_DIR/audio/" 2>/dev/null || true
ls -lh "$FIXTURES_DIR/archives/" 2>/dev/null || true
ls -lh "$FIXTURES_DIR/images/" 2>/dev/null || true
