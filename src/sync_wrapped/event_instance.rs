pub struct EventInstance(pub libfmod::EventInstance);

unsafe impl Sync for EventInstance {}

unsafe impl Send for EventInstance {}
