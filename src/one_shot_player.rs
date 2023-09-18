use libfmod::Studio;

pub struct OneShotPlayer {
    pub studio: Studio,
}

impl OneShotPlayer {
    pub fn play_event(&self, event: &'static str) {
        let event_description = self
            .studio
            .get_event(event)
            .expect("Failed to get event description from the event name supplied.");
        let instance = event_description
            .create_instance()
            .expect("Failed to create event instance.");

        instance.start().expect("Failed to start event.");
        instance.release().expect("Failed to release event.");
    }
}
