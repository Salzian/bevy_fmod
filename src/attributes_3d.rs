use bevy::prelude::Vec3;
use libfmod::{Attributes3d, Vector};

/// Returns the corresponding Attributes3d, which contains all the spatial information FMOD needs
pub fn attributes3d(pos: Vec3, vel: Vec3, fwd: Vec3, up: Vec3) -> Attributes3d {
    Attributes3d {
        position: to_fmod_vec(pos),
        velocity: to_fmod_vec(vel),
        forward: to_fmod_vec(fwd),
        up: to_fmod_vec(up),
    }
}

/// Takes a vector from Bevy coordinate system into the FMOD coordinate system.
/// If FMOD_INIT_3D_RIGHTHANDED is enabled then this is a one-to-one conversion.
fn to_fmod_vec(bevy_vec: Vec3) -> Vector {
    Vector {
        x: bevy_vec.x,
        y: bevy_vec.y,
        z: bevy_vec.z,
    }
}
