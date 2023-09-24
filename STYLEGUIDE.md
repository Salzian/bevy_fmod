# Style guide

This is a living document and will be updated as the project evolves.

## Imports

[Wildcard imports are not allowed][clippy-wildcard-imports] in library code.

Importing from `bevy::prelude` is allowed in library code, however only as direct imports.

Imports from `bevy::prelude::*` are allowed in example code.

## Access modifiers, exports and the `prelude` module

The `bevy_fmod::prelude` module re-exports commonly used types and functions. To keep the usage of the library
as simple as possible, every type, property and function should be private by default.

If the item is needed somewhere else in the crate, but exporting it to the user is not needed, `pub(crate)` should be
used. If the item is needed in the user code, it should be exported in the `prelude` module and re-exported in the standard module way.

### Example

#### Library code

```rust
// lib.rs
pub mod fmod_studio;

// fmod_studio.rs
pub struct FmodStudioPlugin;

// prelude.rs
pub use crate::fmod_studio::FmodStudioPlugin;
```

#### Developer code

The developer has the choice to import the `FmodStudioPlugin` directly from the `fmod_studio` module or from the `prelude` module.

```rust
use bevy_fmod::prelude::*;
// or 
use bevy_fmod::fmod_studio::FmodStudioPlugin;
```

[clippy-wildcard-imports]: https://rust-lang.github.io/rust-clippy/master/index.html#wildcard_imports


