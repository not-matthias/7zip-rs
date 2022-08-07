fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=build.rs");

    // #[cfg(target_os = "windows")]
    let binary = reqwest::blocking::get("https://www.7-zip.org/a/7zr.exe")
        .expect("Failed to download 7zip binary")
        .bytes()
        .expect("Failed to get response as bytes");

    std::fs::write("7zr.exe", binary).expect("Failed to write 7zip binary");

    // Set environment variable
    println!(
        "cargo:rustc-env=7Z_PATH={}",
        std::env::current_dir()
            .unwrap()
            .join("7zr.exe")
            .to_str()
            .unwrap()
    );

    // TODO: Linux https://www.7-zip.org/download.html
}
