use libfmod::Studio;

pub struct OneShotPlayer {
    pub studio: Studio,
}

impl OneShotPlayer {
    pub fn play_event(&self, event: &'static str) {
        let event_description = self.studio.get_event(event).unwrap();
        let instance = event_description.create_instance().unwrap();

        instance.start().unwrap();
        instance.release().unwrap();
    }
}
