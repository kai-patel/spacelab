use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_inspector_egui::{Inspectable, RegisterInspectable, WorldInspectorPlugin};
use bevy_pancam::*;
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Action {
    Thrust,
    Brake,
    RotateLeft,
    RotateRight,
    Left,
    Right,
    ToggleEngine,
}

#[derive(Inspectable, Component)]
struct Orbiting {
    speed: f32,
}

#[derive(Inspectable, Component, Default)]
struct Name(String);

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Component, Default)]
struct Label;

#[derive(Component, Default)]
struct Station;

#[derive(Component, Default)]
struct Ship {
    primary: bool,
}

#[derive(Component, Default)]
struct Planet;

#[derive(Component, Default)]
struct Star;

fn spawn_solar_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn()
        .insert(Star)
        .insert(Name("Sol".to_string()))
        .insert_bundle(SpatialBundle { ..default() })
        .insert_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(50., 50.)).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::ORANGE)),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        })
        .with_children(|star| {
            star.spawn()
                .insert(Planet)
                .insert(Name("Earth".to_string()))
                .insert_bundle(SpatialBundle { ..default() })
                .insert_bundle(MaterialMesh2dBundle {
                    mesh: meshes
                        .add(shape::Quad::new(Vec2::new(10., 10.)).into())
                        .into(),
                    material: materials.add(ColorMaterial::from(Color::BLUE)),
                    transform: Transform::from_translation(Vec3::new(100., 0., 0.)),
                    ..default()
                })
                .insert(Orbiting { speed: 0.001 })
                .with_children(|planet| {
                    planet
                        .spawn()
                        .insert(Station)
                        .insert(Name("ISS".to_string()))
                        .insert_bundle(SpatialBundle { ..default() })
                        .insert_bundle(MaterialMesh2dBundle {
                            mesh: meshes
                                .add(shape::Quad::new(Vec2::new(5., 5.)).into())
                                .into(),
                            material: materials.add(ColorMaterial::from(Color::GRAY)),
                            transform: Transform::from_translation(Vec3::new(25., 0., 0.)),
                            ..default()
                        })
                        .insert(Orbiting { speed: 0.01 })
                        .insert(Label)
                        .with_children(|station| {
                            station.spawn_bundle(Text2dBundle {
                                text: Text::from_section(
                                    "ISS",
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraCode-Retina.ttf"),
                                        font_size: 16.0,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_alignment(TextAlignment::CENTER),
                                transform: Transform::from_translation(Vec3::new(25.0, 0.0, 0.0)),
                                ..default()
                            });
                        });
                });
        });
}

fn spawn_ship(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(3., 3.)).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(Vec3::new(50., 0., 0.)),
            ..default()
        })
        .insert(Ship {
            primary: true,
            d_engine: 4.0,
            d_thruster: 1.0,
        })
        .insert_bundle(InputManagerBundle::<Action> {
            action_state: ActionState::default(),
            input_map: InputMap::new([
                (KeyCode::W, Action::Thrust),
                (KeyCode::S, Action::Brake),
                (KeyCode::A, Action::RotateLeft),
                (KeyCode::D, Action::RotateRight),
                (KeyCode::Comma, Action::Left),
                (KeyCode::Period, Action::Right),
            ]),
        });
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(PanCam {
            grab_buttons: vec![MouseButton::Left, MouseButton::Middle],
            enabled: true,
            zoom_to_cursor: true,
            min_scale: 1.,
            max_scale: Some(40.),
        });
}

fn handle_actions(
    query: Query<&ActionState<Action>, With<Ship>>,
    mut ship_query: Query<(&mut Transform, &Ship)>,
) {
    let action_state = query.single();

    for (mut transform, _) in ship_query.iter_mut().filter(|(_, b)| b.primary) {
        if action_state.pressed(Action::Left) {

            transform.translation += Vec3::new(-10.0, 0.0, 0.0);
        }

        if action_state.pressed(Action::Right) {
            transform.translation += Vec3::new(10.0, 0.0, 0.0);
        }
    }
}

fn draw_orbiting(mut query: Query<(&mut Transform, &Orbiting)>) {
    for (mut transform, orbiting) in query.iter_mut() {
        transform.rotate_around(Vec3::default(), Quat::from_rotation_z(orbiting.speed));
        transform.rotate_local_z(-orbiting.speed);
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(PanCamPlugin::default())
        .add_plugin(InputManagerPlugin::<Action>::default())
        .register_inspectable::<Orbiting>()
        .register_inspectable::<Name>()
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_solar_system)
        .add_startup_system(spawn_ship)
        .add_system(draw_orbiting)
        .add_system(handle_actions)
        .run();
}
