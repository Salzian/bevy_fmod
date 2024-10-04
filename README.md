# bevy_fmod

[![Latest compatible Bevy version](https://img.shields.io/badge/Bevy-0.14.2-232326)](https://crates.io/crates/bevy/0.14.2)
[![bevy_fmod on crates.io](https://img.shields.io/crates/v/bevy_fmod)](https://crates.io/crates/bevy_fmod)

This crate aims to provide an idiomatic [Bevy] plugin for FMOD. This crate
wraps [`libfmod`][libfmod].

```shell
cargo add bevy_fmod
```

## FMOD

FMOD is a cross-platform audio engine that is
used [in many games](https://www.fmod.com/games). It is a commercial product,
with a free license available [for specific terms][FMOD licensing].

### FMOD attribution

This crate is not affiliated with FMOD in any way. It is not endorsed by or
affiliated with Firelight Technologies Pty, Ltd. To use FMOD in your
application, you are required to include attribution by Firelight Technologies'
terms. Learn more [here][FMOD attribution].

## Supported platforms

Currently, this crate is only tested and developed for Windows (non-UWP) and
Linux. More platforms are planned eventually.

MacOS: https://github.com/Salzian/bevy_fmod/issues/2  
Web: https://github.com/Salzian/bevy_fmod/issues/51

Pull requests are welcome.

## Linking the FMOD library

Due to FMOD's licensing, this crate does not include the required FMOD
libraries. You will need to download the appropriate
libraries [here][FMOD libraries download]. This requires a free FMOD account.

### Windows

The linking process on Windows is straight forward. Download and install the
"FMOD Engine" for Windows. When installing, a folder will be created with FMOD
libraries. Copy the followin g files into root of your project:

> [!WARNING]
> TODO: Update this section with the correct file names.

When publishing your game, you will need to include the non-debugging versions
of the libraries in the same directory as the executable.

```
My Game/
├── My Game.exe
├── fmod.dll
└── fmodstudio.dll
```

### MacOS

- Download "FMOD Engine" for MacOS.
- In the dmg file, open the `FMOD Programmers API` folder.
- You will need these files:
    - `api/core/lib/libfmod.dylib`
  - `api/core/lib/libfmodL.dylib` (debugging only)
    - `api/studio/lib/libfmodstudio.dylib`
  - `api/studio/lib/libfmodstudioL.dylib` (debugging only)

Linking on MacOS is a bit different to Windows, as the defaults of the OS are
not as straight forward as Windows. Also, Windows seems to take parent
directories into account. During development, it is sufficient to put the
libraries in the root of your project. When building, the built executable is
contained in the `target/debug` directory. Now on Windows, this doesn't seem to
be a problem, but on MacOS, the executable is unable to find the libraries.

To fix this, you have to pass some flags to cargo during development. Have a
look at this `.cargo/config.toml`:

```toml
[target.aarch64-apple-darwin]
rustflags = [
  "-L", "native=./vendor/fmod",
  "-C", "link-arg=-Wl,-rpath,./vendor/fmod",
]
```

The first line tells cargo where to look for libraries during development. If
you keep the libraries in the root of your project, there is no problem when
building the project. However, when running the executable, it will not find the
libraries, as the executable is in the `target/debug` directory.

I recommend putting the libraries in a known folder like `vendor/fmod` and then
pass the path to cargo using the
`-L` flag. There are different ways to do this, but using the
`.cargo/config.toml` is the most convenient to me.

The second line will determine the rpath of the executable. This is the path
where the executable will look for the libraries. By default, executables will
look in a variety of places, including the directory the executable is in. This
is fine when publishing the game, as you can just use the Windows method and put
the libraries in the same directory as the executable. However, during
development, the executable is in the `target/debug` directory, which gets
generated automatically by cargo and does not contain the libraries. The
`"-C", "link-arg=-Wl,-rpath,./vendor/macos"` flag will tell the executable to
look in the `vendor/fmod` directory of your project for the libraries.

By the end, your project structure should look like this:

```
my_game/
├── .cargo/
│   └── config.toml
├── src/
│   └── <source files>
├── vendor/
│   ├── fmod/
│   │   ├── libfmod.dylib
│   │   ├── libfmodL.dylib
│   │   ├── libfmodstudio.dylib
│   │   └── libfmodstudioL.dylib
│   └── <other external libraries>
└── Cargo.toml
```

### Linux

> [!WARNING]
> This section might be outdated. The approach described here does work, but
> does not align with what is described in the [Windows](#windows)
> and [MacOS](#macos) sections.

Below are the steps for a fairly minimal method to link the libraries. See the
comments in [build.rs](https://github.com/Salzian/bevy_fmod/blob/main/build.rs)
for more information.

- Download the "FMOD Studio" and "FMOD Engine" package for Linux.
- Create a new folder `fmod` in the root of your project.
- Extract the `api` folder into it.
- Copy the contents
  of [build.rs](https://github.com/Salzian/bevy_fmod/blob/main/build.rs) into
  your own build script.

## Examples

To test the examples of this library, clone the repository. FMOD Studio comes
with an Examples project. Open it and select `File > Save as...`. Save the
project as `<bevy_fmod>\assets\audio\demo_project.fspro`. Now, build the
project (
`File > Build`). This will create a folder called
`.\assets\audio\demo_project\Build` which is used by our examples.

Run examples with `cargo run --example <example_name>`. Find the list of
examples in the [Cargo.toml](./Cargo.toml) See the source code of the examples
for more details.

## Live Update

Live update is a way of connecting FMOD Studio to your game as it runs, allowing
you to update and monitor audio content in real
time. [Read more about it here](https://www.fmod.com/docs/2.02/studio/editing-during-live-update.html).

To enable live update, you need to enable the `live-update` feature. While you
can do so in Cargo.toml, we recommend to explicitly enable it with the
`--features` flag. This way, you won't accidentally include it in your release
builds.

```sh
cargo run --example minimal --features live-update
```

[Bevy]: https://bevyengine.org

[FMOD licensing]: https://fmod.com/licensing

[FMOD attribution]: https://fmod.com/attribution

[FMOD libraries download]: https://fmod.com/download#fmodengine

[FMOD revision history]: https://www.fmod.com/docs/2.02/studio/welcome-to-fmod-studio-revision-history.html

[FMOD 2.02.12 img]: https://img.shields.io/badge/FMOD-2.02.12-black

[FMOD 2.02.20 img]: https://img.shields.io/badge/FMOD-2.02.20-black

[libfmod]: https://github.com/lebedec/libfmod

[demo_project]: https://drive.google.com/file/d/13Mxq_jEHXDLuam6M9whNowGUf_KBGKTU/view?usp=sharing

[salzian]: https://salzian.dev

[License img]: https://img.shields.io/badge/License-MIT%20OR%20Apache%202.0-informal

[GitHub releases]: https://github.com/Salzian/bevy_fmod/releases/latest

[GitHub release img]: https://img.shields.io/github/v/release/Salzian/bevy_fmod?filter=v*
