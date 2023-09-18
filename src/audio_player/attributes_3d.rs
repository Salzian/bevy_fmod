use bevy::prelude::Vec3;
use libfmod::{Attributes3d, Vector};

/// Takes the position, velocity and orientation with the Bevy coordinate system
/// and returns the appropriate Attributes3d with the FMOD coordinate system
pub fn attributes3d(pos: Vec3, vel: Vec3, fwd: Vec3, up: Vec3) -> Attributes3d {
    Attributes3d {
        position: Vector {
            x: pos.x,
            y: pos.y,
            z: -pos.z,
        },
        velocity: Vector {
            x: vel.x,
            y: vel.y,
            z: -vel.z,
        },
        forward: Vector {
            x: fwd.x,
            y: fwd.y,
            z: -fwd.z,
        },
        up: Vector {
            x: up.x,
            y: up.y,
            z: -up.z,
        },
    }
}
