use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Default)]
struct Slots {
    eng: u8,
    hab: u8,
    def: u8,
}

impl std::fmt::Display for Slots {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.eng, self.hab, self.def)
    }
}

#[derive(Component, Default)]
struct Station {
    name: String,
    location: String,
    size: u16,
    storage: u16,
    price: u32,
    slots: Slots,
}

struct StationTimer(Timer);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn().insert(Station {
        name: ("ISS".to_string()),
        location: ("Earth".to_string()),
        size: (1_000),
        storage: (1_000),
        ..default()
    });

    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.3, 0.4, 0.5),
            custom_size: Some(Vec2::new(50.0, 100.0)),
            ..default()
        },
        ..default()
    });

    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(50.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        transform: Transform::from_translation(Vec3::new(-100., 0., 0.)),
        ..default()
    });

    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(shape::RegularPolygon::new(50., 6).into()).into(),
        material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
        transform: Transform::from_translation(Vec3::new(-100., 0., 0.)),
        ..default()
    });
}

fn print_stations(time: Res<Time>, mut timer: ResMut<StationTimer>, query: Query<&Station>) {
    if timer.0.tick(time.delta()).just_finished() {
        for station in query.iter() {
            println!(
                "Station {} orbiting {} with size {} and storage {}, costs {} and has {} slots",
                station.name,
                station.location,
                station.size,
                station.storage,
                station.price,
                station.slots
            )
        }
    }
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(StationTimer(Timer::from_seconds(0.0, false)))
        .add_startup_system(setup)
        .add_system(print_stations)
        .run();
}
