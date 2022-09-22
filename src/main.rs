use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_inspector_egui::{Inspectable, RegisterInspectable, WorldInspectorPlugin};
use bevy_pancam::*;
use bevy_prototype_lyon::prelude::*;
use heron::prelude::*;
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

#[derive(Inspectable, Component, Default)]
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

fn spawn_ship(mut commands: Commands) {
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shapes::RegularPolygon {
                sides: 3,
                feature: shapes::RegularPolygonFeature::Radius(3.),
                ..shapes::RegularPolygon::default()
            },
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::DARK_GRAY),
                outline_mode: StrokeMode::new(Color::WHITE, 1.0),
            },
            Transform::from_translation(Vec3::new(50.0, 0., 0.)),
        ))
        .insert(Ship { primary: true })
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::splat(3.0),
            border_radius: None,
        })
        .insert(Velocity::default())
        .insert(Acceleration::default())
        .insert(RotationConstraints::lock())
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
    mut ship_query: Query<(&mut Velocity, &Acceleration, &mut Transform, &Ship)>,
) {
    let action_state = query.single();

    for (mut velocity, _, mut transform, _) in
        ship_query.iter_mut().filter(|(_, _, _, b)| b.primary)
    {
        if action_state.pressed(Action::Left) {
            velocity.linear += transform.left() * 0.01;
        }

        if action_state.pressed(Action::Right) {
            velocity.linear += transform.right() * 0.01;
        }

        if action_state.pressed(Action::Thrust) {
            velocity.linear += transform.up() * 0.1;
        }

        if action_state.pressed(Action::Brake) {
            velocity.linear *= 0.95;
        }

        if action_state.pressed(Action::RotateLeft) {
            transform.rotate_local_z(0.01 * std::f32::consts::PI);
        }

        if action_state.pressed(Action::RotateRight) {
            transform.rotate_local_z(-0.01 * std::f32::consts::PI);
        }
    }
}

fn spawn_orbital_paths(
    mut commands: Commands,
    query: Query<(Option<&Parent>, &GlobalTransform), With<Orbiting>>,
    parent_transform_query: Query<&GlobalTransform>,
    parent_orbiting_query: Query<&Orbiting>,
) {
    println!("Called spawn fn");
    for (parent, transform) in query.iter() {
        println!("Spawning orbit");

        let c = if let Some(p) = parent {
            let parent_transform = parent_transform_query
                .get(p.get())
                .expect("Expected parent entity to have a GlobalTransform")
                .translation();
            Vec2::new(parent_transform.x, parent_transform.y)
        } else {
            Vec2::splat(0.)
        };

        if let Some(p) = parent {
            commands.entity(p.get()).add_children(|par| {
                par.spawn_bundle(GeometryBuilder::build_as(
                    &shapes::Circle {
                        radius: transform.translation().distance(Vec3::new(c.x, c.y, -1.0)),
                        center: c,
                    },
                    DrawMode::Stroke(StrokeMode::new(Color::WHITE, 2.)),
                    Transform::default(),
                ))
                .insert(Orbiting {
                    speed: if let Ok(orbiting) = parent_orbiting_query.get(p.get()) {
                        orbiting.speed
                    } else {
                        0.
                    },
                });
            });
        } else {
            commands.spawn_bundle(GeometryBuilder::build_as(
                &shapes::Circle {
                    radius: transform.translation().distance(Vec3::new(c.x, c.y, -1.0)),
                    center: c,
                },
                DrawMode::Stroke(StrokeMode::new(Color::WHITE, 2.)),
                Transform::default(),
            ));
        }

        println!("Orbit spawned!");
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
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(ShapePlugin)
        .register_inspectable::<Orbiting>()
        .register_inspectable::<Name>()
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_solar_system)
        .add_startup_system(spawn_ship)
        .add_startup_system_to_stage(StartupStage::PostStartup, spawn_orbital_paths)
        .add_system(draw_orbiting)
        .add_system(handle_actions)
        .run();
}
