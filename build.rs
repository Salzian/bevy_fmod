/// This build script is needed to run the examples on Linux.
/// Feel free to copy this to your own Bevy project.
/// Note that currently it is not possible to use it together with the Bevy dynamic_linking feature.

fn main() {
    #[cfg(all(target_os = "linux", target_arch = "x86"))]
    {
        println!("cargo:rustc-link-search=/usr/lib/i386-linux-gnu/fmod-api/");
        println!("cargo:rustc-env=LD_LIBRARY_PATH=/usr/lib/i386-linux-gnu/fmod-api/");
    }

    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    {
        println!("cargo:rustc-link-search=/usr/lib/x86_64-linux-gnu/fmod-api/");
        println!("cargo:rustc-env=LD_LIBRARY_PATH=/usr/lib/x86_64-linux-gnu/fmod-api/");
    }

    #[cfg(all(target_os = "linux", target_arch = "arm"))]
    {
        println!("cargo:rustc-link-search=/usr/lib/arm-linux-gnueabihf/fmod-api/");
        println!("cargo:rustc-env=LD_LIBRARY_PATH=/usr/lib/arm-linux-gnueabihf/fmod-api/");
    }

    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    {
        println!("cargo:rustc-link-search=/usr/lib/aarch64-linux-gnu/fmod-api/");
        println!("cargo:rustc-env=LD_LIBRARY_PATH=/usr/lib/aarch64-linux-gnu/fmod-api/");
    }
}
