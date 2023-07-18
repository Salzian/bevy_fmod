extern crate anyhow;
extern crate bevy;
extern crate bevy_mod_sysfail;
extern crate libfmod;

mod play_sound_event;
mod plugin;

pub use crate::play_sound_event::PlaySoundEvent;
pub use crate::plugin::FmodPlugin;
