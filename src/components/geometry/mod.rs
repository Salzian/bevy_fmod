use std::os::raw::c_int;
use crate::FmodStudio;
use crate::attributes_3d::ToFmodVecExt;
use bevy::{log::error, prelude::*};
use bevy::mesh::Indices;
use libfmod::ffi::{FMOD_Geometry_AddPolygon, FMOD_BOOL, FMOD_GEOMETRY, FMOD_VECTOR};
use libfmod::Vector;

mod visibility;

#[derive(Component, Deref, DerefMut)]
#[require(Transform)]
pub struct Geometry(libfmod::Geometry);

impl Drop for Geometry {
    fn drop(&mut self) {
        if let Err(err) = self.release() {
            error!("Failed to release geometry: {}", err);
        }
    }
}

struct GeometryPlugin {
    /// Whether to activate the geometry automatically based on its [`Visibility`].
    activate_automatic_active_based_on_visibility: bool,
}

impl Plugin for GeometryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_position));

        if self.activate_automatic_active_based_on_visibility {
            app.add_systems(
                Update,
                (
                    visibility::update_active_based_on_visibility,
                    visibility::update_active_based_on_inherent_visibility,
                ),
            );
        }
    }
}

fn update_position(geometries: Query<(&Geometry, &Transform), Changed<Transform>>) -> Result {
    for (geometry, transform) in geometries.iter() {
        geometry.set_position(transform.translation.to_fmod_vec())?;
        geometry.set_rotation(
            Some((transform.rotation * Vec3::Z).to_fmod_vec()),
            Some((transform.rotation * Vec3::Y).to_fmod_vec()),
        )?;
        geometry.set_scale(transform.scale.to_fmod_vec())?;
    }

    Ok(())
}


impl Geometry {
    fn from_mesh(
        mesh: &Mesh,
        studio: &FmodStudio,
    ) -> Result<Geometry> {
        let system = studio.get_core_system()?;

        if let Some(indices) = mesh.indices() {
            match indices {
                Indices::U16(indices) => {
                    Self::from_indices(indices, studio)
                }
                Indices::U32(indices) => {
                    Self::from_indices(indices, studio)
                }
            }
        } else {
            Ok(Geometry(system.create_geometry(0, 0)?))
        }
    }

    fn from_indices<T: Into<u32> + Copy>(
        indices: &[T],
        studio: &FmodStudio,
    ) -> Result<Geometry> {
        let system = studio.get_core_system()?;

        let polygons = indices.chunks(3);
        let geometry = system
            .create_geometry(
                polygons.len() as i32,
                indices.iter().count() as i32)?;

        for (i, polygon) in polygons.enumerate() {
            let polygon: Vec<u32> = polygon.iter().map(|&x| x.into()).collect();

            unsafe {
                FMOD_Geometry_AddPolygon(
                    geometry.as_mut_ptr(),
                    1.0,
                    1.0,
                    FMOD_BOOL::from(false),
                    3,
                    polygon.as_ptr() as *const FMOD_VECTOR,
                    i as *mut c_int,
                );
            }
        }

        Ok(Geometry(geometry))
    }
} 

#[cfg(feature = "avian3d")]
mod avian3d {
    use crate::components::Geometry;
    use avian3d::prelude::Collider;

    impl From<avian3d::collision::collider::Collider> for Geometry {
        fn from(value: Collider) -> Self {
            value.shape()
        }
    }
}
