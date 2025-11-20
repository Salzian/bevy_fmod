use super::Geometry;
use bevy::prelude::{*};

pub(super) fn update_active_based_on_visibility(
    geometries: Query<(&Geometry, &Visibility), Changed<Visibility>>,
) -> Result {
    for (geometry, visibility) in geometries.iter() {
        match visibility {
            Visibility::Hidden => {
                geometry.set_active(false)?;
            }
            Visibility::Visible => {
                geometry.set_active(true)?;
            }
            Visibility::Inherited => {}
        }
    }

    Ok(())
}

pub(super) fn update_active_based_on_inherent_visibility(
    geometries: Query<(&Geometry, &InheritedVisibility), Changed<InheritedVisibility>>,
) -> Result {
    for (geometry, inherited_visibility) in geometries.iter() {
        geometry.set_active(**inherited_visibility)?;
    }

    Ok(())
}
