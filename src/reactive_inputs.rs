use bevy::{
    prelude::*,
    input::mouse::{MouseButtonInput, MouseMotion, MouseButton},
    math::{cubic_splines::CubicCurve, vec3}
};
use bevy::input::ButtonState;

pub struct ReactiveInputsPlugin;

#[derive(Resource)]
pub struct Paused(bool);

impl Plugin for ReactiveInputsPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Paused(false))
            .add_systems(Startup, setup)
            .add_systems(Update, animate_cube);
    }
}

#[derive(Component)]
pub struct Curve(CubicCurve<Vec3>);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut pause_state: ResMut<Paused>
) {
    let points = [[
        vec3(-6., 2., 0.),
        vec3(12., 8., 0.),
        vec3(-12., 8., 0.),
        vec3(6., 2., 0.)
    ]];

    let bezier = Bezier::new(points).to_curve();

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Cube::default().into()),
            material: materials.add(Color::ORANGE.into()),
            transform: Transform::from_translation(points[0][0]),
            ..default()
        },
        Curve(bezier),
    ));

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8., 16., 8.),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(50.).into()),
        material: materials.add(Color::SILVER.into()),
        ..default()
    });

    // The camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 6., 12.).looking_at(Vec3::new(0., 3., 0.), Vec3::Y),
        ..default()
    });
}

pub fn animate_cube(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Curve)>,
    mut gizmos: Gizmos,
    mut pause_state: ResMut<Paused>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
) {
    for event in mouse_button_input_events.iter() {
        if event.button == MouseButton::Left && event.state == ButtonState::Pressed {
            pause_state.0 = !pause_state.0;
        }
    }

    // Only animate if not paused
    if !pause_state.0 {
        let t = (time.elapsed_seconds().sin() + 1.) / 2.;

        for (mut transform, cubic_curve) in &mut query {
            // Draw the curve
            gizmos.linestrip(cubic_curve.0.iter_positions(50), Color::WHITE);
            // position takes a point from the curve where 0 is the initial point
            // and 1 is the last point
            transform.translation = cubic_curve.0.position(t);
        }
    }
}