use crate::attributes_3d::attributes3d;
use crate::components::user_property::{UserProperty, UserPropertyType};
use crate::components::velocity::Velocity;
use bevy::log::{error, warn};
use bevy::math::Vec3;
use bevy::prelude::{Component, Deref, DerefMut, GlobalTransform, Query};
use libfmod::{EventDescription, EventInstance, StopMode};

/// See the [`Velocity`] component for information on enabling the Doppler effect.
#[derive(Component, Deref, DerefMut, Builder)]
#[builder(pattern = "owned")]
pub struct AudioSource {
    #[deref]
    pub event_instance: EventInstance,
    #[builder(default = "StopMode::Immediate")]
    pub(crate) despawn_stop_mode: StopMode,
}

impl From<EventDescription> for AudioSource {
    /// Creates an [`AudioSource`] from an [`EventDescription`]. The despawn stop mode is inferred from
    /// the user property "despawn-stop-mode" on the event description. If the user property is not
    /// found, the despawn stop mode is set to [`StopMode::Immediate`].
    fn from(value: EventDescription) -> Self {
        let builder =
            AudioSourceBuilder::default().event_instance(value.create_instance().unwrap());

        let stop_mode = builder
            .parse_despawn_stop_mode_user_property()
            .unwrap_or(StopMode::Immediate);

        builder.despawn_stop_mode(stop_mode).build().unwrap()
    }
}

impl From<EventDescription> for AudioSourceBuilder {
    /// Crreates a new [`AudioSourceBuilder`] with the given event instance. The returned builder can
    /// be used to further configure the audio source. Call [`AudioSourceBuilder::build`] to create the
    /// [`AudioSource`].
    fn from(value: EventDescription) -> Self {
        Self::default().event_instance(value.create_instance().unwrap())
    }
}

const DESPAWN_STOP_MODE_USER_PROPERTY_NAME: &str = "despawn-stop-mode";

impl AudioSourceBuilder {
    /// Parses the "despawn-stop-mode" user property on the event description to infer the despawn
    /// stop mode. If the user property is not found, the despawn stop mode is set to
    /// [`StopMode::Immediate`].
    fn parse_despawn_stop_mode_user_property(&self) -> Option<StopMode> {
        match self.event_instance {
            None => {
                error!(
                "The AudioSourceBuilder must have an event_instance to infer the stop mode from \
                its event description."
            );
                None
            }
            Some(event_instance) => {
                let event_description = event_instance.get_description().unwrap();

                match event_description.get_user_property(DESPAWN_STOP_MODE_USER_PROPERTY_NAME) {
                    Ok(libfmod_stop_mode_property) => {
                        let stop_mode_property = UserProperty::from(libfmod_stop_mode_property);
                        let property_type = stop_mode_property.get_value();
                        let value = match property_type {
                            UserPropertyType::String(string_value) => string_value,
                            _ => {
                                warn!(
                        "The '{DESPAWN_STOP_MODE_USER_PROPERTY_NAME}' user property must be a \
                        string. Got: {:?}",
                        property_type
                    );

                                return None;
                            }
                        };

                        return match value.as_str() {
                            "immediate" => Some(StopMode::Immediate),
                            "allow-fadeout" => Some(StopMode::AllowFadeout),
                            _ => {
                                warn!(
                        "The '{DESPAWN_STOP_MODE_USER_PROPERTY_NAME}' user property must be either \
                        'immediate' or 'allow-fadeout'. Got: {:?}",
                        value
                    );

                                None
                            }
                        };
                    }
                    Err(error) => {
                        error!(
                            "Failed to read {DESPAWN_STOP_MODE_USER_PROPERTY_NAME} user property.",
                        );
                        error!("{error}");

                        None
                    }
                }
            }
        }
    }
}

impl AudioSource {
    pub(crate) fn update_3d_attributes(
        mut query: Query<(&AudioSource, &GlobalTransform, Option<&Velocity>)>,
    ) {
        query
            .iter_mut()
            .for_each(|(audio_source, transform, vel_component)| {
                let mut velocity = Vec3::ZERO;

                if let Some(vel_component) = vel_component {
                    velocity = vel_component.current_velocity;
                }

                audio_source
                    .set_3d_attributes(attributes3d(
                        transform.translation(),
                        velocity,
                        *transform.forward(),
                        *transform.up(),
                    ))
                    .unwrap();
            });
    }
}

impl AudioSource {
    #[deprecated = "Use `AudioSource::get_volume` instead."]
    pub fn volume(&self) -> f32 {
        self.get_volume().unwrap().0
    }

    #[deprecated = "Use `AudioSource::set_volume` instead."]
    pub fn set_volume(&self, volume: f32) {
        self.event_instance.set_volume(volume).unwrap();
    }

    #[deprecated = "Use `AudioSource::get_pitch` instead."]
    pub fn speed(&self) -> f32 {
        self.get_pitch().unwrap().0
    }

    #[deprecated = "Use `AudioSource::set_pitch` instead."]
    pub fn set_speed(&self, speed: f32) {
        self.set_pitch(speed).unwrap();
    }

    #[deprecated = "Use `AudioSource::start` instead."]
    pub fn play(&self) {
        if self.get_paused().unwrap() {
            self.set_paused(false).unwrap();
        } else {
            self.start().unwrap();
        }
    }

    #[deprecated = "Use `AudioSource::set_paused(bool)` instead."]
    pub fn pause(&self) {
        self.set_paused(true).unwrap();
    }

    #[deprecated = "Use `AudioSource::get_paused` instead."]
    pub fn is_paused(&self) -> bool {
        self.get_paused().unwrap()
    }

    pub fn toggle(&self) {
        self.event_instance
            .set_paused(!self.event_instance.get_paused().unwrap())
            .unwrap();
    }
}
