/// This build script is needed to run the examples on Linux.
/// Feel free to copy this to your own Bevy project.
///
/// Be aware that on Linux you also need to set LD_LIBRARY_PATH to include the libraries.
/// There are a few ways to do it:
///
/// If you are working with an IDE it's easiest to add this to the run configuration.
/// For example, in Rust Rover you set `Environment variables` to `LD_LIBRARY_PATH=./fmod/api/core/lib/x86_64:./fmod/api/studio/lib/x86_64`.
///
/// If you are running your executable directly (no IDE, no cargo) see https://www.hpc.dtu.dk/?page_id=1180

fn main() {
    #[cfg(target_os = "linux")]
    {
        let mut target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();

        // Account for the naming mismatch between FMOD library folders and the target architecture
        if target_arch == "aarch64" {
            target_arch = "arm64".into();
        }

        let api_loc = [
            format!("./fmod/api/core/lib/{target_arch}"),
            format!("./fmod/api/studio/lib/{target_arch}"),
        ];
        for loc in api_loc {
            println!("cargo:rustc-link-search={loc}");
            println!("cargo:rustc-env=LD_RUN_PATH={loc}");
        }
    }
}
