use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    log::LogSettings,
    prelude::*,
    sprite::MaterialMesh2dBundle,
    utils::HashMap,
    winit::WinitSettings,
};
use bevy_easings::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_inspector_egui::{Inspectable, RegisterInspectable, WorldInspectorPlugin};
use bevy_pancam::*;
use bevy_prototype_lyon::prelude::*;
use egui_extras::TableBuilder;
use heron::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Default, Debug)]
struct UiState {
    space: bool,
    cargo: bool,
}

impl UiState {
    fn new() -> Self {
        UiState {
            space: true,
            ..default()
        }
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Action {
    Thrust,
    Brake,
    RotateLeft,
    RotateRight,
    Left,
    Right,
    Dock,
    Cargo,
}

struct DockEvent(Entity);

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

#[derive(Component, Default)]
struct Dockable {
    is_docked: bool,
}

#[derive(Component, Default)]
struct DisplayCargo;

#[derive(Component, Default)]
struct CargoHold {
    items: HashMap<Item, u64>,
}

impl CargoHold {
    fn new(items: HashMap<Item, u64>) -> Self {
        CargoHold { items }
    }

    fn store(&mut self, item: Item, quantity: u64) {
        *self.items.entry(item).or_insert(0) += quantity;
    }

    fn remove(&mut self, item: Item, quantity: u64) {
        self.items.entry(item).and_modify(|e| *e -= quantity);
        self.items.retain(|_, v| *v != 0);
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
struct Item {
    name: String,
    description: String,
}

impl Item {
    fn new(name: &str, description: &str) -> Self {
        Item {
            name: name.to_string(),
            description: description.to_string(),
        }
    }
}

fn spawn_solar_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    debug!("spawn_solar_system");
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
    debug!("Solar system spawned");
}

fn spawn_ship(mut commands: Commands) {
    debug!("spawn_ship");
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
        .insert(Dockable { is_docked: false })
        .insert(CargoHold::default())
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
                (KeyCode::C, Action::Cargo),
            ])
            .insert_chord([KeyCode::LShift, KeyCode::D], Action::Dock)
            .build(),
        });
    debug!("Ship spawned");
}

fn dock_to_nearest(
    mut commands: Commands,
    query: Query<(&GlobalTransform, Entity), With<Station>>,
    mut ship_query: Query<(&GlobalTransform, &mut Dockable, Entity), With<Ship>>,
    mut dock_event: EventReader<DockEvent>,
) {
    for dock in dock_event.iter() {
        let (ship_transform, mut dockable, ship_entity) = ship_query
            .get_mut(dock.0)
            .expect("Expected docking ship to exist");

        let ship_location = ship_transform.translation();

        let ds = query
            .iter()
            .filter(|(station_transform, _)| {
                station_transform.translation().distance(ship_location) < 50.0
            })
            .min_by(|(a, _), (b, _)| {
                a.translation()
                    .distance(ship_location)
                    .total_cmp(&b.translation().distance(ship_location))
            });

        if let Some((_, nearest)) = ds {
            debug!("Found nearest {:?}", nearest);
            commands
                .entity(ship_entity)
                .insert(Orbiting { speed: 0.01 })
                .remove::<RigidBody>();
            commands.entity(nearest).add_child(ship_entity);
            dockable.is_docked = true;
        } else {
            debug!("No stations nearby");
        }
    }

    dock_event.clear();
}

fn spawn_camera(mut commands: Commands) {
    debug!("Spawn camera");
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(PanCam {
            grab_buttons: vec![MouseButton::Left, MouseButton::Middle],
            enabled: true,
            zoom_to_cursor: true,
            min_scale: 1.,
            max_scale: Some(40.),
        });
    debug!("Camera spawned");
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                size: Size::new(Val::Percent(100.0), Val::Percent(20.0)),
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                    ..default()
                },
                border: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            color: Color::rgb(0.4, 0.4, 0.4).into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        display: Display::Flex,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        ..default()
                    },
                    interaction: Interaction::Clicked,
                    color: Color::GREEN.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        "Cargo",
                        TextStyle {
                            font: asset_server.load("fonts/FiraCode-Retina.ttf"),
                            font_size: 36.0,
                            color: Color::PURPLE,
                        },
                    ));
                })
                .insert(DisplayCargo);
        })
        .with_children(|parent| {
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    display: Display::Flex,
                    ..default()
                },
                color: Color::BLUE.into(),
                ..default()
            });
        })
        .with_children(|parent| {
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    display: Display::Flex,
                    ..default()
                },
                color: Color::RED.into(),
                ..default()
            });
        });
}

fn handle_ui_click(
    mut query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<DisplayCargo>)>,
    mut ui_state: ResMut<UiState>,
) {
    let interaction = match query.get_single_mut() {
        Ok(x) => x,
        _ => return,
    };

    if *interaction == Interaction::Clicked {
        ui_state.cargo = !ui_state.cargo;
        ui_state.set_changed();
    }
}

fn handle_cargo_button_color(
    mut query: Query<&mut UiColor, With<DisplayCargo>>,
    ui_state: Res<UiState>,
) {
    if ui_state.is_changed() {
        if let Ok(mut color) = query.get_single_mut() {
            *color = match ui_state.cargo {
                true => Color::RED.into(),
                false => Color::GREEN.into(),
            };
        }
    }
}

fn ship_cargo_ui(
    mut egui_ctx: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    mut query: Query<(&mut CargoHold, &Ship)>,
) {
    let (mut cargo_hold, _) = query
        .iter_mut()
        .filter(|(_, ship)| ship.primary)
        .nth(0)
        .expect("Expected one and only one primary ship to exist");

    egui::Window::new("Ship Cargo")
        .vscroll(true)
        .open(&mut ui_state.cargo)
        .resizable(true)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.vertical(|ui| {
                ui.heading("Cargo Manifest");
                TableBuilder::new(ui)
                    .resizable(true)
                    .striped(true)
                    .column(egui_extras::Size::remainder())
                    .column(egui_extras::Size::remainder())
                    .column(egui_extras::Size::remainder())
                    .header(20.0, |mut header| {
                        header.col(|ui| {
                            ui.heading("Name");
                        });
                        header.col(|ui| {
                            ui.heading("Quantity");
                        });
                        header.col(|ui| {
                            ui.heading("Description");
                        });
                    })
                    .body(|mut body| {
                        cargo_hold.items.iter().for_each(|(k, v)| {
                            body.row(30.0, |mut row| {
                                row.col(|ui| {
                                    ui.label(&k.name);
                                });
                                row.col(|ui| {
                                    ui.label(v.to_string());
                                });
                                row.col(|ui| {
                                    ui.label(&k.description);
                                });
                            });
                        });
                    });
                if ui.button("Add item").clicked() {
                    cargo_hold.store(Item::new("Iron Ore", "Some iron ore"), 1);
                    debug!("{:?}", cargo_hold.items);
                }

                if ui.button("Remove item").clicked() {
                    cargo_hold.remove(Item::new("Iron Ore", "Some iron ore"), 1);
                    debug!("{:?}", cargo_hold.items);
                }
            });
        });

    ui_state.set_changed();
}

fn handle_actions(
    mut commands: Commands,
    query: Query<&ActionState<Action>, With<Ship>>,
    mut ship_query: Query<(
        Entity,
        &mut Velocity,
        &mut Dockable,
        &mut Transform,
        Option<&Parent>,
        &Ship,
    )>,
    mut ui_state: ResMut<UiState>,
    mut dock_event: EventWriter<DockEvent>,
) {
    let action_state = query.single();

    for (entity, mut velocity, mut dockable, mut transform, parent, _) in
        ship_query.iter_mut().filter(|(_, _, _, _, _, b)| b.primary)
    {
        if !dockable.is_docked {
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

            if action_state.just_pressed(Action::Cargo) {
                ui_state.cargo = !ui_state.cargo;
                ui_state.set_changed();
            }
        }

        if action_state.just_pressed(Action::Dock) {
            // If not docked, then attempt to dock
            if !dockable.is_docked {
                dock_event.send(DockEvent(entity));
            } else {
                // If already docked, undock

                // Ensure parent component exists;
                let parent_component = parent.expect("Expected docked ship to have parent");

                // Get parent entity ID
                let parent_entity = parent_component.get();

                // Remove Orbiting component and add back physics for ship
                commands
                    .entity(entity)
                    .remove::<Orbiting>()
                    .insert(RigidBody::Dynamic);

                // Remove ship from station children
                commands.entity(parent_entity).remove_children(&[entity]);

                // Undocking completed
                dockable.is_docked = false;
            }
        }
    }
}

fn spawn_orbital_paths(
    mut commands: Commands,
    query: Query<(Option<&Parent>, &GlobalTransform), With<Orbiting>>,
    parent_transform_query: Query<&GlobalTransform>,
) {
    debug!("spawn_orbital_paths");
    // Iterate through all GlobalTransforms which Orbit
    for (parent, transform) in query.iter() {
        // Get (x, y) global position of parent, or default to (0, 0)
        let c = if let Some(p) = parent {
            let parent_transform = parent_transform_query
                .get(p.get())
                .expect("Expected parent entity to have a GlobalTransform")
                .translation();
            Vec2::new(parent_transform.x, parent_transform.y)
        } else {
            Vec2::splat(0.)
        };

        // Add orbital path as child to parent of the orbiting body
        if let Some(p) = parent {
            commands.entity(p.get()).add_children(|par| {
                par.spawn_bundle(GeometryBuilder::build_as(
                    &shapes::Circle {
                        radius: transform.translation().distance(Vec3::new(c.x, c.y, 0.0)),
                        center: Vec2::splat(0.),
                    },
                    DrawMode::Stroke(StrokeMode::new(Color::WHITE, 2.)),
                    Transform::default(),
                ));
            });
        } else {
            commands.spawn_bundle(GeometryBuilder::build_as(
                &shapes::Circle {
                    radius: transform.translation().distance(Vec3::new(c.x, c.y, -1.0)),
                    center: Vec2::splat(0.),
                },
                DrawMode::Stroke(StrokeMode::new(Color::WHITE, 2.)),
                Transform::default(),
            ));
        }
    }
    debug!("Orbits spawned!");
}

fn draw_orbiting(mut query: Query<(&mut Transform, &Orbiting)>, ui_state: Res<UiState>) {
    if ui_state.space {
        for (mut transform, orbiting) in query.iter_mut() {
            transform.rotate_around(Vec3::default(), Quat::from_rotation_z(orbiting.speed));
            transform.rotate_local_z(-orbiting.speed);
        }
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(LogSettings {
            filter: "info,wgpu_core=warn,wgpu_hal=warn,spacelab=debug".into(),
            level: bevy::log::Level::DEBUG,
        })
        .insert_resource(WinitSettings::game())
        .insert_resource(UiState::new())
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(PanCamPlugin::default())
        .add_plugin(InputManagerPlugin::<Action>::default())
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(ShapePlugin)
        .add_plugin(EguiPlugin)
        .add_plugin(EasingsPlugin)
        .register_inspectable::<Orbiting>()
        .register_inspectable::<Name>()
        .add_event::<DockEvent>()
        .add_startup_system_to_stage(StartupStage::PreStartup, spawn_camera)
        .add_startup_system_to_stage(StartupStage::PreStartup, setup_ui)
        .add_startup_system(spawn_solar_system)
        .add_startup_system(spawn_ship)
        .add_startup_system_to_stage(StartupStage::PostStartup, spawn_orbital_paths)
        .add_system(draw_orbiting)
        .add_system(handle_actions)
        .add_system(handle_ui_click)
        .add_system(ship_cargo_ui)
        .add_system(handle_cargo_button_color)
        .add_system(dock_to_nearest)
        .run();
}
