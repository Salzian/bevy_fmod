# bevy_fmod

[![bevy_fmod on crates.io](https://img.shields.io/crates/v/bevy_fmod)](https://crates.io/crates/bevy_fmod)
[![Latest compatible Bevy version](https://img.shields.io/badge/Bevy-0.17-black)](https://crates.io/crates/bevy/0.17.0)
[![Supported FMOD version](https://img.shields.io/badge/FMOD-2.02.22-black)](https://github.com/lebedec/libfmod?tab=readme-ov-file#installation)

This crate aims to provide an idiomatic [Bevy] plugin for FMOD. This crate
wraps [`libfmod`][libfmod] and is therefore constrained to the same version of
FMOD that it uses.

Version `0.9.0` of this crate is compatible with Bevy `0.17` and FMOD `2.02.22`.

> [!WARNING]
>
> This crate is not affiliated with FMOD in any way. It is not endorsed by or
> affiliated with Firelight Technologies Pty, Ltd. To use FMOD in your
> application, you are required to include attribution by Firelight Technologies'
> terms. Learn more [here][FMOD attribution].

## Getting started

### Linking the FMOD library

To use this crate, you need to link the FMOD library.

Due to FMOD's licensing, this crate does not include the required FMOD libraries.
You will need to download the appropriate libraries [here][FMOD libraries download].
This requires a free FMOD account.

For maximum compatibility, we recommend to download the same version of FMOD
that `libfmod` uses. The latest supported version is `2.02.22`.

The FMOD Studio download contains the FMOD Studio desktop application,
which is mostly used by who do sound design for your game. To use this crate,
you need to download the **FMOD Engine** package. This package contains
**both** the FMOD Studio API library and the FMOD Engine API library.

<details>

<summary>Windows</summary>

#### Windows

Download and install FMOD Engine for Windows. When installing, 
a folder will be created with FMOD libraries.
Copy the following files into root of your project:

- `api/core/lib/x64/fmod.dll`
- `api/core/lib/x64/fmod_vc.lib`
- `api/studio/lib/x64/fmodstudio.dll`
- `api/studio/lib/x64/fmodstudio_vc.lib`

When publishing your game, you will need to include these libraries in the same
directory as the executable.

The final game will ship with the following structure:

```text
My Game/
├── My Game.exe
├── fmod.dll
├── fmod_vc.lib
├── fmodstudio.dll
└── fmodstudio_vc.lib
```

</details>

<details>

<summary>MacOS</summary>

#### MacOS

- Download "FMOD Engine" for MacOS.
- In the dmg file, open the `FMOD Programmers API` folder.
- You will need these files:
    - `api/core/lib/libfmod.dylib`
    - `api/studio/lib/libfmodstudio.dylib`

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
`"-C", "link-arg=-Wl,-rpath,./vendor/fmod"` flag will tell the executable to
look in the `vendor/fmod` directory of your project for the libraries.

By the end, your project structure should look like this:

```text
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

</details>

<details>

<summary>Linux</summary>

#### Linux

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
- Copy the contents of [build.rs](https://github.com/Salzian/bevy_fmod/blob/main/build.rs) into your own build script.

</details>

## Examples

To get started fast, I recommend you to check out the [minimal example](./examples/minimal.rs).
It contains the minimum amount of code to get audio playing.

To test the examples of this library, clone the repository. FMOD Studio comes
with an Examples project. Open it and select `File > Save as...`. Save the
project as `<bevy_fmod>\assets\audio\demo_project.fspro`. Now, build the
project (
`File > Build`). This will create a folder called
`.\assets\audio\demo_project\Build` which is used by our examples.

## About FMOD

FMOD is a cross-platform audio engine that is
used [in many games](https://www.fmod.com/games). It is a commercial product,
with a free license available [for specific terms][FMOD licensing].

### Supported platforms

List of supported / tested platforms for this crate. Other platforms might work,
but have not been tested. List of platforms taken from the
[FMOD documentation][FMOD Platform Details].

| Platform                         | Support | Issue                                            |
|----------------------------------|---------|--------------------------------------------------|
| Windows                          | ✅       |                                                  |
| Mac                              | ✅       |                                                  |
| Linux                            | ✅       |                                                  |
| iOS                              | ❌       |                                                  |
| Android                          | ❌       |                                                  |
| Open Harmony                     | ❌       |                                                  |
| Universal Windows Platform (UWP) | ❌       |                                                  |
| HTML5                            | ❌ ️     | <https://github.com/Salzian/bevy_fmod/issues/51> |

> [!NOTE]
>
> Pull requests are welcome.

## Features

### Live Update

Live update is a way of connecting FMOD Studio to your game as it runs, allowing
you to update and monitor audio content in real
time. [Read more about it here](https://www.fmod.com/docs/2.02/studio/editing-during-live-update.html).

To enable live update, you need to enable the `live-update` feature. While you
can do so in Cargo.toml, we recommend explicitly enabling it with the
`--features` flag. This way, you won't accidentally include it in your release
builds.

```sh
cargo run --example minimal --features live-update
```

## Utilities

With version `0.9.0`, this crate includes a few utilities that are not part of
the main API, but make sense to have in the context of a bevy game. To read 
more about them, check out the [utilities](https://docs.rs/bevy_fmod/latest/bevy_fmod/utilities/index.html)
module in the documentation.

Utilities are part of the `utilities` feature, which is enabled by default.

[Bevy]: https://bevyengine.org

[FMOD licensing]: https://fmod.com/licensing

[FMOD attribution]: https://fmod.com/attribution

[FMOD libraries download]: https://fmod.com/download#fmodengine

[FMOD Platform Details]: https://www.fmod.com/docs/2.02/api/platforms.html

[libfmod]: https://github.com/lebedec/libfmod
