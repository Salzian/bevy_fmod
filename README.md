# bevy_fmod

![Latest compatible Bevy version](https://img.shields.io/badge/Bevy-0.11.0-informational)
![Latest compatible FMOD version](https://img.shields.io/badge/FMOD-2.02.12-informational)  
![GitHub](https://img.shields.io/github/license/salzian/bevy_fmod)
![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/salzian/bevy_fmod)

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

This repository includes [examples](./examples) and a FMOD demo project.

## Versioning

Version 0.0.1 would be the first usable MVP somewhat recommended for use.  
Version 1.0.0 would be a hypothetical stable release. But that's still a long way to go.

| bevy_fmod | Bevy                                      | FMOD                                                  |
|-----------|-------------------------------------------|-------------------------------------------------------|
| 0.1.0     | [![0.11.0][Bevy 0.11.0 img]][Bevy 0.11.0] | [![2.02.12][FMOD 2.02.12 img]][FMOD revision history] |
| 0.0.1     | [![0.10.0][Bevy 0.10.0 img]][Bevy 0.10.0] | [![2.02.12][FMOD 2.02.12 img]][FMOD revision history] |

[Bevy]: https://bevyengine.org

[Bevy 0.10.0]: https://bevyengine.org/news/bevy-0-10/

[Bevy 0.11.0]: https://bevyengine.org/news/bevy-0-11/

[Bevy 0.10.0 img]: https://img.shields.io/badge/Bevy-0.10.0-informational

[Bevy 0.11.0 img]: https://img.shields.io/badge/Bevy-0.11.0-informational

[FMOD licensing]: https://fmod.com/licensing

[FMOD attribution]: https://fmod.com/attribution

[FMOD libraries download]: https://fmod.com/download#fmodengine

[FMOD revision history]: https://www.fmod.com/docs/2.02/studio/welcome-to-fmod-studio-revision-history.html

[FMOD 2.02.12 img]: https://img.shields.io/badge/FMOD-2.02.12-informational

[libfmod]: https://github.com/lebedec/libfmod

[demo_project]: examples/demo_project

[salzian]: https://salzian.dev
