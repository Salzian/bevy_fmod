use std::env;
use std::fs::{self, canonicalize, metadata};
use std::path::Path;

#[cfg(target_os = "windows")]
use std::os::windows;

#[cfg(target_os = "unix")]
use std::os::unix;
use std::time::SystemTime;

/// Bundles dynamic libraries from a path into the build folder.
///
/// # Example
///
/// Vendor folder includes the FMOD libraries like this:
/// - vendor
///   - fmod.dll
///   - fmodstudio.dll
///
/// ```rust
/// use bevy_fmod_dynamic_bundler::bundle_libraries;
///
/// bundle_libraries("./vendor".into());
/// ```
pub fn bundle_libraries(path: &str) {
    let profile = env::var("PROFILE").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();
    let vendor_dir = canonicalize(path)
        .unwrap_or_else(|error| panic!("Failed to canonicalize path `{}`: {}", path, error));

    for entry in fs::read_dir(vendor_dir)
        .unwrap_or_else(|error| panic!("Failed to read directory `{}`: {}", path, error))
    {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();

        let metadata = metadata(&path).unwrap_or_else(|error| {
            panic!(
                "Failed to read metadata for {}: {}",
                path.to_str().unwrap_or("unknown"),
                error
            )
        });

        let file_type = metadata.file_type();

        if !(file_type.is_file()) {
            continue;
        }

        #[cfg(target_os = "windows")]
        {
            let needed_files = vec!["dll", "lib"];
            if !needed_files.contains(
                &path
                    .extension()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default(),
            ) {
                continue;
            }
        }

        #[cfg(target_os = "macos")]
        if !path.extension().unwrap_or_default().eq("dylib") {
            continue;
        }

        #[cfg(target_os = "linux")]
        if !path.extension().unwrap_or_default().eq("so") {
            continue;
        }

        let destination_path =
            Path::new(&out_dir).join(path.file_name().expect("Failed to get file name"));

        if profile == "debug" {
            if destination_path.try_exists().unwrap_or_else(|error| {
                panic!(
                    "Failed to check if {} exists: {}",
                    destination_path.to_str().unwrap_or("unknown"),
                    error
                )
            }) {
                continue;
            }

            // Create symlink in debug mode
            #[cfg(unix)]
            unix::fs::symlink(&path, &destination_path).expect(
                format!(
                    "Failed to create symlink for {}",
                    path.to_str().unwrap_or("unknown")
                )
                .as_str(),
            );

            #[cfg(windows)]
            windows::fs::symlink_file(&path, &destination_path).unwrap_or_else(|error| {
                panic!(
                    "Failed to create symlink for {}: {}",
                    path.to_str().unwrap_or("unknown"),
                    error
                )
            });
        } else {
            if destination_path.exists() {
                let destination_metadata =
                    fs::metadata(&destination_path).unwrap_or_else(|error| {
                        panic!(
                            "Failed to read metadata for {}: {}",
                            destination_path.to_str().unwrap_or("unknown"),
                            error
                        )
                    });

                // Compare if the destination file is older than the source file.
                // In case any metadata retrieval fails, we assume the file is older and overwrite it.
                if destination_metadata
                    .modified()
                    .unwrap_or(SystemTime::UNIX_EPOCH)
                    >= metadata.modified().unwrap_or(SystemTime::now())
                {
                    continue;
                } else {
                    fs::remove_file(&destination_path).unwrap_or_else(|error| {
                        panic!(
                            "Failed to remove {}: {}",
                            destination_path.to_str().unwrap_or("unknown"),
                            error
                        )
                    });
                }
            }

            // Copy file in release mode
            fs::copy(&path, &destination_path).unwrap_or_else(|error| {
                panic!(
                    "Failed to copy {} to {}: {}",
                    path.to_str().unwrap_or("unknown"),
                    destination_path.to_str().unwrap_or("unknown"),
                    error
                )
            });
        }
    }
}
