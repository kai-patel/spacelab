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

#[derive(Component, Default, Debug)]
struct Anchor(f32, f32);

impl From<&Anchor> for Vec3 {
    fn from(a: &Anchor) -> Self {
        Vec3 {
            x: a.0,
            y: a.1,
            z: 0.,
        }
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
    anchor: Anchor,
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

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn draw_stations(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    query: Query<(&Name, &Anchor), With<Station>>,
) {
    for (name, anchor) in query.iter() {
        commands.spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
            transform: Transform::from_translation(Vec3::from(anchor)).with_scale(Vec3::splat(8.)),
            material: materials.add(ColorMaterial::from(Color::GREEN)),
            ..default()
        });

        commands.spawn_bundle(Text2dBundle {
            text: Text::from_section(
                name.0.to_string(),
                TextStyle {
                    font_size: 20.0,
                    font: asset_server.load("fonts/FiraCode-Retina.ttf"),
                    color: Color::WHITE,
                    ..default()
                },
            ),
            ..default()
        });
    }
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
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_station)
        .add_startup_system(spawn_ship)
        .add_system(draw_stations)
        .add_system(print_stations)
        .add_system(print_ships)
        .run();
}
