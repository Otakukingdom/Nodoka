fn main() {
    // Try pkg-config first (most reliable on Linux/macOS)
    #[cfg(not(target_os = "windows"))]
    {
        if pkg_config::probe_library("libvlc").is_ok() {
            return;
        }
    }

    // Platform-specific fallbacks
    #[cfg(target_os = "macos")]
    {
        // Check environment variable override
        if let Ok(vlc_path) = std::env::var("VLC_LIB_PATH") {
            println!("cargo:rustc-link-search={vlc_path}");
        } else {
            // Multiple fallback paths for different VLC installations
            println!("cargo:rustc-link-search=/Applications/VLC.app/Contents/MacOS/lib");
            println!("cargo:rustc-link-search=/usr/local/lib");
            println!("cargo:rustc-link-search=/opt/homebrew/lib");
        }
        println!("cargo:rustc-link-lib=dylib=vlc");
    }

    #[cfg(target_os = "windows")]
    {
        // Check environment variable override
        if let Ok(vlc_path) = std::env::var("VLC_LIB_PATH") {
            println!("cargo:rustc-link-search={vlc_path}");
        } else {
            // Multiple fallback paths for different VLC installations
            println!("cargo:rustc-link-search=C:/Program Files/VideoLAN/VLC");
            println!("cargo:rustc-link-search=C:/Program Files (x86)/VideoLAN/VLC");
        }
        println!("cargo:rustc-link-lib=dylib=libvlc");
    }

    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-lib=dylib=vlc");
    }
}
