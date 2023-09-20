use crate::fmod_studio::FmodStudio;
use bevy::prelude::Resource;

#[derive(Resource)]
pub struct OneShotPlayer(pub FmodStudio);

impl OneShotPlayer {
    pub fn play_event(&self, event: &'static str) {
        let event_description = self
            .0
             .0
            .get_event(event)
            .expect("Failed to get event description from the event name supplied.");
        let instance = event_description
            .create_instance()
            .expect("Failed to create event instance.");

        instance.start().expect("Failed to start event.");
        instance.release().expect("Failed to release event.");
    }
}
