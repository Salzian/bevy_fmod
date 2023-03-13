extern crate bevy;
extern crate libfmod;
extern crate bevy_mod_sysfail;
extern crate anyhow;

mod play_sound_event;
mod plugin;

pub use crate::play_sound_event::PlaySoundEvent;
pub use crate::plugin::FmodPlugin;
