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
        let mut search_paths = Vec::new();

        if let Ok(vlc_path) = std::env::var("VLC_LIB_PATH") {
            search_paths.push(std::path::PathBuf::from(vlc_path));
        }
        if let Ok(sdk_path) = std::env::var("VLC_SDK_PATH") {
            search_paths.push(std::path::PathBuf::from(sdk_path));
        }

        let mut found = false;
        for base in search_paths {
            let libvlc_lib = base.join("libvlc.lib");
            let sdk_lib = base.join("sdk").join("lib").join("libvlc.lib");
            let lib_dir = base.join("lib").join("libvlc.lib");

            if libvlc_lib.exists() {
                println!("cargo:rustc-link-search={}", base.display());
                found = true;
                break;
            }
            if sdk_lib.exists() {
                println!(
                    "cargo:rustc-link-search={}",
                    base.join("sdk").join("lib").display()
                );
                found = true;
                break;
            }
            if lib_dir.exists() {
                println!("cargo:rustc-link-search={}", base.join("lib").display());
                found = true;
                break;
            }
        }

        if !found {
            println!(
                "cargo:warning=libvlc.lib not found. Install the VLC SDK and set VLC_SDK_PATH \
to its root (or set VLC_LIB_PATH to the SDK 'lib' directory)."
            );
            std::process::exit(1);
        }
        println!("cargo:rustc-link-lib=dylib=libvlc");
    }

    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-lib=dylib=vlc");
    }
}
