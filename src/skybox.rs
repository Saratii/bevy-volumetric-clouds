use core::f32::consts::PI;

use bevy::{light::light_consts::lux::FULL_DAYLIGHT, prelude::*};

use crate::CloudCamera;

#[derive(Component)]
pub(crate) struct SkyboxPlane {
    pub orig_translation: Vec3,
    pub orig_rotation: Quat,
}

pub(crate) struct SkyboxMaterials<M: Material> {
    pub nx: MeshMaterial3d<M>,
    pub ny: MeshMaterial3d<M>,
    pub nz: MeshMaterial3d<M>,
    pub px: MeshMaterial3d<M>,
    pub py: MeshMaterial3d<M>,
    pub pz: MeshMaterial3d<M>,
}

impl<M: Material> SkyboxMaterials<M> {
    pub fn from_one_material(material: MeshMaterial3d<M>) -> Self {
        Self {
            nx: material.clone(),
            ny: material.clone(),
            nz: material.clone(),
            px: material.clone(),
            py: material.clone(),
            pz: material.clone(),
        }
    }
}

/// Spawn 6 sides of a cube with front faces facing inwards, representing the sky
///
/// Make sure the `standard_materials` are unlit.
pub(crate) fn init_skybox_mesh<M: Material>(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    standard_materials: SkyboxMaterials<M>,
) {
    let box_size = 1.0;

    let mesh = meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(box_size)));

    // negative x
    let rotation_nx = Quat::from_rotation_z(-PI * 0.5) * Quat::from_rotation_y(PI * 0.5);
    commands.spawn((
        Mesh3d(mesh.clone()),
        standard_materials.nx,
        Transform::from_translation(Vec3::new(-box_size, 0.0, 0.0)).with_rotation(rotation_nx),
        SkyboxPlane {
            orig_translation: Vec3::new(-box_size, 0.0, 0.0),
            orig_rotation: rotation_nx,
        },
    ));

    // negative y
    commands.spawn((
        Mesh3d(mesh.clone()),
        standard_materials.ny,
        Transform::from_translation(Vec3::new(0.0, -box_size, 0.0)),
        SkyboxPlane {
            orig_translation: Vec3::new(0.0, -box_size, 0.0),
            orig_rotation: Quat::IDENTITY,
        },
    ));

    // negative z
    let rotation_nz = Quat::from_rotation_x(PI * 0.5);
    commands.spawn((
        Mesh3d(mesh.clone()),
        standard_materials.nz,
        Transform::from_translation(Vec3::new(0.0, 0.0, -box_size)).with_rotation(rotation_nz),
        SkyboxPlane {
            orig_translation: Vec3::new(0.0, 0.0, -box_size),
            orig_rotation: rotation_nz,
        },
    ));

    // positive x
    let rotation_px = Quat::from_rotation_z(PI * 0.5) * Quat::from_rotation_y(-PI * 0.5);
    commands.spawn((
        Mesh3d(mesh.clone()),
        standard_materials.px,
        Transform::from_translation(Vec3::new(box_size, 0.0, 0.0)).with_rotation(rotation_px),
        SkyboxPlane {
            orig_translation: Vec3::new(box_size, 0.0, 0.0),
            orig_rotation: rotation_px,
        },
    ));

    // positive y
    let rotation_py = Quat::from_rotation_z(PI) * Quat::from_rotation_y(PI);
    commands.spawn((
        Mesh3d(mesh.clone()),
        standard_materials.py,
        Transform::from_translation(Vec3::new(0.0, box_size, 0.0)).with_rotation(rotation_py),
        SkyboxPlane {
            orig_translation: Vec3::new(0.0, box_size, 0.0),
            orig_rotation: rotation_py,
        },
    ));

    // positive z
    let rotation_pz = Quat::from_rotation_x(-PI * 0.5) * Quat::from_rotation_y(PI);
    commands.spawn((
        Mesh3d(mesh.clone()),
        standard_materials.pz,
        Transform::from_translation(Vec3::new(0.0, 0.0, box_size)).with_rotation(rotation_pz),
        SkyboxPlane {
            orig_translation: Vec3::new(0.0, 0.0, box_size),
            orig_rotation: rotation_pz,
        },
    ));
}

pub(crate) fn setup_daylight(mut commands: Commands) {
    commands.spawn((
        Transform::from_xyz(1.0, 1.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        DirectionalLight {
            illuminance: FULL_DAYLIGHT,
            ..default()
        },
    ));
}

pub(crate) fn update_skybox_transform(
    camera: Single<
        (&GlobalTransform, &Camera, &Projection),
        (Without<SkyboxPlane>, With<CloudCamera>),
    >,
    mut skybox: Query<(&mut Transform, &SkyboxPlane)>,
) {
    let far = match camera.2 {
        Projection::Perspective(pers) => pers.far,
        _ => {
            panic!("unexpected projection")
        }
    };
    let scale = far * 4.0;

    for (mut transform, plane) in skybox.iter_mut() {
        transform.scale = Vec3::splat(scale);
        transform.translation = camera.0.translation() + plane.orig_translation * scale;
        transform.rotation = plane.orig_rotation;
    }
}
