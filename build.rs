fn main() {
    // Platform-specific VLC library paths
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-search=/Applications/VLC.app/Contents/MacOS/lib");
        println!("cargo:rustc-link-lib=dylib=vlc");
    }

    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-search=C:/Program Files/VideoLAN/VLC");
        println!("cargo:rustc-link-lib=dylib=libvlc");
    }

    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-lib=dylib=vlc");
    }
}
