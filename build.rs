use fs::copy;
use std::env::var;
use std::path::Path;
use std::{env, fs};

use fs::metadata;
use std::env::var;
use std::fs;
use std::fs::copy;
use std::path::Path;

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
            format!("./vendor/fmod/linux/api/core/lib/{target_arch}"),
            format!("./vendor/fmod/linux/api/studio/lib/{target_arch}"),
        ];
        for loc in api_loc {
            println!("cargo:rustc-link-search={loc}");
            println!("cargo:rustc-env=LD_RUN_PATH={loc}");
        }
    }

    #[cfg(target_os = "macos")]
    {
        let out_dir = var("OUT_DIR").unwrap();
        let target_dir = Path::new(&out_dir).parent().unwrap().parent().unwrap();

        copy_lib_files(target_dir);

        configure_rustc_for_macos(target_dir);
    }

    #[cfg(target_os = "macos")]
    {
        let out_dir = var("OUT_DIR").unwrap();

        let source_directory = Path::new("./vendor/fmod/macos/api/");
        let target_directory = Path::new(&out_dir);

        copy_lib_files(source_directory, target_directory);

        configure_rustc_for_macos(target_directory);
    }
}

#[cfg(target_os = "macos")]
fn copy_lib_files(source_directory: &Path, target_directory: &Path) {
    // Adapt these paths as necessary
    let fmod_library_path = source_directory.join("core/lib/libfmod.dylib");
    let fmodstudio_library_path = source_directory.join("studio/lib/libfmodstudio.dylib");

    // Determine the full paths to the destination files.
    let fmod_library_destination = target_directory.join("libfmod.dylib");
    let fmodstudio_library_destination = target_directory.join("libfmodstudio.dylib");

    // Copy the libraries only if the source files are newer than the destination files.
    if let Some(false) = is_newer(&fmod_library_path, &fmod_library_destination) {
        copy(&fmod_library_path, &fmod_library_destination).expect("Could not copy libfmod.dylib");
        println!("Copied libfmod.dylib to {:?}", fmod_library_destination);
    }

    if let Some(false) = is_newer(&fmodstudio_library_path, &fmodstudio_library_destination) {
        copy(&fmodstudio_library_path, &fmodstudio_library_destination)
            .expect("Could not copy libfmodstudio.dylib");
        println!(
            "Copied libfmodstudio.dylib to {:?}",
            fmodstudio_library_destination
        );
    }
}

fn is_newer(source_path: &Path, destination_path: &Path) -> Option<bool> {
    let source_metadata = metadata(source_path)
        .unwrap_or_else(|_| panic!("Could not get metadata of source file: {:?}", source_path));
    let destination_metadata = metadata(destination_path);

    println!("Source metadata: {:?}", source_metadata);
    println!("Destination metadata: {:?}", destination_metadata);

    let modified_src = source_metadata.modified().unwrap_or_else(|_| {
        panic!(
            "Could not get modified time of source file: {:?}",
            source_path
        )
    });

    if let Ok(metadata) = destination_metadata {
        let modified_dest = metadata.modified().unwrap_or_else(|_| {
            panic!(
                "Could not get modified time of destination file: {:?}",
                destination_path
            )
        });

        println!("Source modified: {:?}", modified_src);
        Some(modified_src > modified_dest)
    } else {
        // destination file does not exist, so the source file is considered as "newer"
        println!("Destination file does not exist: {:?}", destination_path);
        None
    }
}

#[cfg(target_os = "macos")]
fn configure_rustc_for_macos(target_directory: &Path) {
    println!("cargo:rustc-link-lib=dylib=fmod");
    println!("cargo:rustc-link-lib=dylib=fmodstudio");
    println!("cargo:rustc-link-search=native=/Users/ffritzsche/Workspace/salzian/games/bevy_fmod/vendor/fmod/macos/api/core/lib");
    println!("cargo:rustc-link-search=native=/Users/ffritzsche/Workspace/salzian/games/bevy_fmod/vendor/fmod/macos/api/studio/lib");
}

#[cfg(target_os = "macos")]
fn copy_lib_files(target_dir: &Path) {
    // Adapt these paths as necessary
    let src_fmod = "./vendor/fmod/api/core/lib/libfmod.dylib";
    let src_fmodstudio = "./vendor/fmod/api/studio/lib/libfmodstudio.dylib";

    // Determine the full paths to the destination files
    let dest_fmod = target_dir.join("libfmod.dylib");
    let dest_fmodstudio = target_dir.join("libfmodstudio.dylib");

    // Copy the libraries only if the source files are newer than the destination files
    if is_newer(&src_fmod, &dest_fmod) {
        copy(&src_fmod, &dest_fmod).expect("Could not copy libfmod.dylib");
    }
    if is_newer(&src_fmodstudio, &dest_fmodstudio) {
        copy(&src_fmodstudio, &dest_fmodstudio).expect("Could not copy libfmodstudio.dylib");
    }
}

fn is_newer(src_path: &str, dest_path: &Path) -> bool {
    let metadata_src = fs::metadata(src_path).expect("Could not get metadata of source file");
    let metadata_dest = fs::metadata(dest_path);

    let modified_src = metadata_src
        .modified()
        .expect("Could not get modified time of source file");

    match metadata_dest {
        Ok(metadata) => {
            let modified_dest = metadata
                .modified()
                .expect("Could not get modified time of destination file");
            modified_src > modified_dest
        }
        Err(_) => true, // destination file does not exist, so the source file is considered as "newer"
    }
}

#[cfg(target_os = "macos")]
fn configure_rustc_for_macos(target_dir: &Path) {
    println!("cargo:rustc-link-lib=dylib=fmod");
    println!("cargo:rustc-link-lib=dylib=fmodstudio");
    println!("cargo:rustc-link-search=native={}", target_dir.display());
}
