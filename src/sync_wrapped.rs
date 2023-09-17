unsafe impl Sync for EventInstance {}
unsafe impl Send for EventInstance {}

pub struct EventInstance(pub libfmod::EventInstance);
