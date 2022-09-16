use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Component)]
struct Orbiting {
    speed: f32,
}

#[derive(Component, Default)]
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
struct Ship;

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
                                transform: Transform::from_translation(Vec3::new(10.0, 0.0, 0.0)),
                                ..default()
                            });
                        });
                });
        });
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
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
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_solar_system)
        .add_system(draw_orbiting)
        .run();
}
