use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Component, Default)]
enum Type {
    #[default]
    STATION,
    SHIP,
}

#[derive(Component, Default)]
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
struct Name(String);

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Component, Default)]
struct Location(String);

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Component, Default)]
struct Size(u16);

impl std::fmt::Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Component, Default)]
struct Storage(u16);

impl std::fmt::Display for Storage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Component, Default)]
struct Price(u64);

impl std::fmt::Display for Price {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Component, Default)]
struct Station;

#[derive(Bundle, Default)]
struct VesselBundle {
    name: Name,
    location: Location,
    size: Size,
    storage: Storage,
    price: Price,
    slots: Slots,
    vessel_type: Type,
}
struct StationTimer(Timer);

fn spawn_station(mut commands: Commands) {
    commands
        .spawn()
        .insert(Station)
        .insert_bundle(VesselBundle {
            name: Name("ISS".to_string()),
            location: Location("Earth".to_string()),
            size: Size(1_000),
            storage: Storage(1_000),
            price: Price(150_000_000_000),
            vessel_type: Type::STATION,
            ..default()
        });
}

#[derive(Component, Default)]
struct Heading(Option<(f64, f64, f64)>);

impl std::fmt::Display for Heading {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some((a, b, c)) => write!(f, "({}, {}, {})", a, b, c),
            None => write!(f, "Currently docked"),
        }
    }
}

#[derive(Component, Default)]
struct Docked(bool);

#[derive(Bundle, Default)]
struct ShipBundle {
    heading: Heading,
    docked: Docked,
}

#[derive(Component, Default)]
struct Ship;

struct ShipTimer(Timer);

fn spawn_ship(mut commands: Commands) {
    commands
        .spawn()
        .insert(Ship)
        .insert_bundle(ShipBundle {
            heading: Heading(None),
            docked: Docked(false),
        })
        .insert_bundle(VesselBundle {
            name: Name("Space Shuttle".to_string()),
            location: Location("ISS".to_string()),
            size: Size(965),
            storage: Storage(29_000),
            price: Price(1_700_000_000),
            vessel_type: Type::SHIP,
            ..default()
        });
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
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

fn print_stations(
    time: Res<Time>,
    mut timer: ResMut<StationTimer>,
    query: Query<(&Name, &Location, &Size, &Storage, &Price, &Slots), With<Station>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for (name, location, size, storage, price, slots) in query.iter() {
            println!(
                "Station {} orbiting {} with size {} and storage {}, costs {} and has {} slots",
                name, location, size, storage, price, slots
            )
        }
    }
}

fn print_ships(
    time: Res<Time>,
    mut timer: ResMut<ShipTimer>,
    query: Query<
        (
            &Name,
            &Location,
            &Size,
            &Storage,
            &Price,
            &Slots,
            &Heading,
            &Docked,
        ),
        With<Ship>,
    >,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for (name, location, size, storage, price, slots, heading, docked) in query.iter() {
            if docked.0 {
                println!(
                    "Ship {} last docked at station {} with heading {}, size {} and storage {}, costs {} and has {} slots",
                    name, location, heading, size, storage, price, slots
                )
            } else {
                println!(
                    "Ship {} docked with station {} with size {} and storage {}, costs {} and has {} slots",
                    name, location, size, storage, price, slots
                )
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(StationTimer(Timer::from_seconds(0.0, false)))
        .insert_resource(ShipTimer(Timer::from_seconds(0.0, false)))
        .add_startup_system(setup)
        .add_startup_system(spawn_station)
        .add_startup_system(spawn_ship)
        .add_system(print_stations)
        .add_system(print_ships)
        .run();
}
