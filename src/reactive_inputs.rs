use bevy::{
    prelude::*,
    math::{cubic_splines::CubicCurve, vec3}
};

#[derive(Component)]
pub struct Curve(CubicCurve<Vec3>);

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_system(Update, animate_cube)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let points = [[
        vec3(-6., 2., 0.),
        vec3(12., 8., 0.),
        vec3(-12., 8., 0.),
        vec3(6., 2., 0.)
    ]];

    let bezier = Bezier::new(points).to_curve();
}