//! A demo example featuring shadows working cloud background.
//!
//! - Enable the `fly_camera` feature to be able to control the camera with keyboard and mouse.
//! - Enable the `debug` feature to be able to control the clouds settings using an `egui` UI.
use bevy::prelude::*;
use bevy::render::view::Hdr;
#[cfg(feature = "debug")]
use bevy_egui::EguiPlugin;
#[cfg(feature = "fly_camera")]
use bevy_volumetric_clouds::fly_camera::{FlyCam, FlyCameraPlugin, MovementSettings};
use bevy_volumetric_clouds::{CloudCamera, CloudsPlugin};

#[derive(Component)]
struct Rotatable;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            CloudsPlugin,
            #[cfg(feature = "fly_camera")]
            FlyCameraPlugin,
            #[cfg(feature = "debug")]
            EguiPlugin::default(),
        ))
        .add_systems(Startup, (setup, setup_daylight))
        .add_systems(Update, (close_on_esc, rotate_cube))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut std_materials: ResMut<Assets<StandardMaterial>>,
) {
    #[cfg(feature = "fly_camera")]
    commands.insert_resource(MovementSettings { speed: 15.0 });
    commands.spawn((
        Camera3d::default(),
        Hdr,
        CloudCamera,
        #[cfg(feature = "fly_camera")]
        FlyCam,
        Transform::from_translation(Vec3::new(50.0, 12.0, 0.0))
            .looking_at(Vec3::new(-40.0, 0.0, 0.0), Vec3::Y),
    ));
    //spawn rotating cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid {
            half_size: Vec3::new(5.0, 5.0, 5.0),
        })),
        MeshMaterial3d(std_materials.add(Color::srgb_u8(0, 0, 255))),
        Transform::from_translation(Vec3::new(0.0, 12., 0.0)),
        Rotatable,
    ));
    //spawn ground plane
    commands.spawn((
        Mesh3d(meshes.add(Cuboid {
            half_size: Vec3::new(20.0, 0.2, 20.0),
        })),
        MeshMaterial3d(std_materials.add(Color::srgb_u8(255, 0, 0))),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
    ));
}

fn setup_daylight(mut commands: Commands) {
    commands.spawn((
        Transform::from_xyz(0.0, 1.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        DirectionalLight {
            illuminance: 8000.0,
            shadows_enabled: true,
            ..default()
        },
    ));
}

fn rotate_cube(mut query: Query<&mut Transform, With<Rotatable>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.rotate_y(0.5 * time.delta_secs());
        transform.rotate_x(0.5 * time.delta_secs());
        transform.rotate_z(0.5 * time.delta_secs());
    }
}

fn close_on_esc(
    mut commands: Commands,
    focused_window: Single<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if focused_window.1.focused && input.just_pressed(KeyCode::Escape) {
        commands.entity(focused_window.0).despawn();
    }
}
