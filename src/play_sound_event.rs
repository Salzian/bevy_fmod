use bevy::prelude::Event;

// TODO: Replace this with an EventDescription?
#[derive(Event)]
pub struct PlaySoundEvent(pub &'static str);
