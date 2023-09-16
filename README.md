# bevy_fmod

![Latest compatible Bevy version][Bevy 0.11.2 img]
![Latest compatible FMOD version][FMOD 2.02.12 img]
![License][License img]
![GitHub release][GitHub release img]

This crate aims to provide an idiomatic [Bevy] plugin for FMOD. This crate wraps [`libfmod`][libfmod].

## FMOD

FMOD is a cross-platform audio engine that is used in many games. It is a commercial product, with a free license
available [for specific terms][FMOD licensing].

### FMOD attribution

This crate is not affiliated with FMOD in any way. It is not endorsed by or affiliated with Firelight Technologies Pty,
Ltd. To use FMOD in your application, you are required to include attribution by Firelight Technologies' terms.
Learn more [here][FMOD attribution].

## Supported platforms

Currently, this crate is only tested and developed for Windows (non-UWP). More platforms are not planned for the near
future. Pull requests are welcome.

## External dependencies

This crate does not bundle the required FMOD libraries. You will need to download the appropriate
libraries [here][FMOD libraries download].
This requires a free FMOD account.

- Download the "FMOD Engine" package for Windows. Make sure to select [a compatible version](#versioning).
- Install the package.
- You need the following 4 files in the root of your rust project:
    - `api/core/lib/x64/fmod.dll`
    - `api/core/lib/x64/fmod_vc.lib`: **rename to `fmod.lib`**
    - `api/studio/lib/x64/fmodstudio.dll`
    - `api/studio/lib/x64/fmodstudio_vc.lib`: **rename to `fmodstudio.lib`**

## Usage

```toml
[dependencies]
bevy_fmod = { git = "https://github.com/Salzian/bevy_fmod.git", tag = "v0.1.0" }
```

## Examples

This repository includes a `hello_world` example. In order to run it, you need
to [download the demo project][demo_project] (Google Drive link) and place it into the root of the
project: `./demo_project`.

## Versioning

| bevy_fmod | Bevy                                    | FMOD (tested version, newer may work)                 |
|-----------|-----------------------------------------|-------------------------------------------------------|
| 0.2.0     | [![0.11.2][Bevy 0.11.2 img]][Bevy 0.11] | [![2.02.12][FMOD 2.02.12 img]][FMOD revision history] |
| 0.1.0     | [![0.10.0][Bevy 0.10.0 img]][Bevy 0.10] | [![2.02.12][FMOD 2.02.12 img]][FMOD revision history] |
| 0.0.1     | [![0.10.0][Bevy 0.10.0 img]][Bevy 0.10] | [![2.02.12][FMOD 2.02.12 img]][FMOD revision history] |

[Bevy]: https://bevyengine.org

[Bevy 0.10]: https://bevyengine.org/news/bevy-0-10/

[Bevy 0.11]: https://bevyengine.org/news/bevy-0-11/

[Bevy 0.10.0 img]: https://img.shields.io/badge/Bevy-0.10.0-232326

[Bevy 0.11.2 img]: https://img.shields.io/badge/Bevy-0.11.2-232326

[FMOD licensing]: https://fmod.com/licensing

[FMOD attribution]: https://fmod.com/attribution

[FMOD libraries download]: https://fmod.com/download#fmodengine

[FMOD revision history]: https://www.fmod.com/docs/2.02/studio/welcome-to-fmod-studio-revision-history.html

[FMOD 2.02.12 img]: https://img.shields.io/badge/FMOD-2.02.12-black

[libfmod]: https://github.com/lebedec/libfmod

[demo_project]: https://drive.google.com/file/d/13Mxq_jEHXDLuam6M9whNowGUf_KBGKTU/view?usp=sharing

[salzian]: https://salzian.dev

[License img]: https://img.shields.io/badge/License-MIT%20OR%20Apache%202.0-informal

[GitHub release img]: https://img.shields.io/github/v/release/Salzian/bevy_fmod?filter=v*