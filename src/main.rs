// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

mod animation;
mod core;
mod menu;
mod prelude;
mod screen;
mod theme;
mod util;

use crate::prelude::*;
use avian2d::math::Vector;

pub fn plugin(app: &mut App) {
    app
        .insert_resource(DefaultFriction(Friction::new(0.)))
        .insert_resource(DefaultRestitution(
            Restitution::new(1.),
        ));

    // Add core plugins.
    app.add_plugins(core::plugin);

    // Add other plugins.
    app.add_plugins((
        animation::plugin,
        menu::plugin,
        screen::plugin,
        theme::plugin,
        util::plugin,
    ));

    app.add_systems(Startup, setup);
    app.add_systems(Update, move_players);
}

fn main() -> AppExit {
    run()
}

// TODO: Workaround for <https://github.com/DioxusLabs/dioxus/issues/4160>.
fn run() -> AppExit {
    use bevy::prelude::*;

    fn main() -> AppExit {
        App::new().add_plugins(plugin).run()
    }

    main()
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Player1;

#[derive(Component)]
struct Player2;

#[derive(Component)]
struct Ball;

fn setup(mut commands: Commands) {
    let width = 20.0;
    let height = 200.0;
    commands.spawn((
        Player {},
        Player1 {},
        Name::new("Player1"),
        Collider::rectangle(width, height),
        Transform::from_xyz(-100.0, 0.0, 0.0),
        RigidBody::Dynamic,
        LinearVelocity::default(),
        LockedAxes::ALL_LOCKED.unlock_translation_y(),
        Sprite::from_color(
            Srgba::from_vec3(Vec3::splat(0.5)),
            Vec2 {
                x: width,
                y: height,
            },
        ),
        Restitution {
            coefficient: 1.0,
            combine_rule: CoefficientCombine::Max,
        },
        Friction {
            dynamic_coefficient: 0.0,
            static_coefficient: 0.0,
            combine_rule: CoefficientCombine::Max,
        },
        LinearDamping(0.0),
        AngularDamping(0.0),
        GravityScale(0.0)
    ));
    commands.spawn((
        Player {},
        Player2 {},
        Name::new("Player2"),
        Collider::rectangle(width, height),
        Transform::from_xyz(100.0, 0.0, 0.0),
        RigidBody::Dynamic,
        LinearVelocity::default(),
        LockedAxes::ALL_LOCKED.unlock_translation_y(),
        Sprite::from_color(
            Srgba::from_vec3(Vec3::splat(0.5)),
            Vec2 {
                x: width,
                y: height,
            },
        ),
        Restitution {
            coefficient: 1.0,
            combine_rule: CoefficientCombine::Max,
        },
        Friction {
            dynamic_coefficient: 0.0,
            static_coefficient: 0.0,
            combine_rule: CoefficientCombine::Max,
        },
        LinearDamping(0.0),
        AngularDamping(0.0),
        GravityScale(0.0)
    ));
    commands.spawn((
        Name::new("Ball"),
        RigidBody::Dynamic,
        Collider::circle(width / 2.0),
        LinearVelocity(Vector::new(1000.0, 0.0)),
        Sprite::from_color(Srgba::from_vec3(Vec3::splat(0.5)), Vec2::splat(width)),
        Ball {},
        LockedAxes::ROTATION_LOCKED,
        Restitution {
            coefficient: 1.0,
            combine_rule: CoefficientCombine::Max,
        },
        Friction {
            dynamic_coefficient: 0.0,
            static_coefficient: 0.0,
            combine_rule: CoefficientCombine::Max,
        },
        LinearDamping(0.0),
        AngularDamping(0.0),
        GravityScale(0.0)
    ));
}

fn move_players(
    keys: Res<ButtonInput<KeyCode>>,
    mut transform_query: Query<&mut LinearVelocity, With<Player>>,
    player1_id: Single<Entity, With<Player1>>,
    player2_id: Single<Entity, With<Player2>>,
    time: Res<Time>,
) {
    let speed = time.delta_secs() * 5000.0;

    let mut p1_speed = 0.0;
    p1_speed += if keys.pressed(KeyCode::KeyW) { speed } else { 0.0 };
    p1_speed += if keys.pressed(KeyCode::KeyS) { -speed } else { 0.0 };
    println!("{p1_speed}");
    let mut velocity = r!(transform_query.get_mut(*player1_id));
    velocity.y = p1_speed;

    let mut p2_speed = 0.0;
    p2_speed += if keys.pressed(KeyCode::ArrowUp) { speed } else { 0.0 };
    p2_speed += if keys.pressed(KeyCode::ArrowDown) { -speed } else { 0.0 };
    println!("{p2_speed}");
    let mut velocity = r!(transform_query.get_mut(*player2_id));
    velocity.y = p2_speed;
}

// fn move_ball(time: Res<Time>, transform_query: Query<&Ball>) {
// 
// }

// TODO: Workaround for <https://github.com/DioxusLabs/dioxus/issues/4160>.
#[cfg(feature = "bevy_mod_debugdump")]
fn debug() {
    // Silence dead code warnings while writing debugging code.
    #![allow(dead_code)]

    use bevy::app::MainScheduleOrder;
    use bevy::ecs::schedule::LogLevel;
    use bevy::ecs::schedule::ScheduleLabel;
    use bevy::prelude::*;
    use bevy_mod_debugdump::schedule_graph_dot;
    use tiny_bail::prelude::*;

    fn main() {
        let app = &mut App::new();
        app.add_plugins(plugin);

        //print_schedule_graph(app, Update);
        build_schedules(app, LogLevel::Ignore);
        list_schedules(app);
        list_main_schedules(app);
        list_systems(app, Update);
    }

    /// Usage: Disable logging with RUST_LOG=off, then pipe the output into `dot`.
    /// Example: `RUST_LOG=off bevy run --bin debug --features bevy_mod_debugdump | dot -Tsvg | feh -`.
    fn print_schedule_graph(app: &mut App, label: impl ScheduleLabel) {
        let dot = schedule_graph_dot(app, label, &default());
        println!("{dot}");
    }

    fn build_schedules(app: &mut App, ambiguity_detection: LogLevel) {
        r!(app
            .world_mut()
            .try_resource_scope::<Schedules, _>(|world, mut schedules| {
                for (_, schedule) in schedules.iter_mut() {
                    let label = schedule.label();

                    // Enable ambiguity detection.
                    let mut settings = schedule.get_build_settings();
                    settings.ambiguity_detection = ambiguity_detection.clone();
                    schedule.set_build_settings(settings);

                    // Build schedule.
                    let graph = schedule.graph_mut();
                    graph.initialize(world);
                    c!(graph.build_schedule(world, label.intern(), &default()));
                }
            }));
    }

    fn list_schedules(app: &mut App) {
        let mut labels = r!(app.world().get_resource::<Schedules>())
            .iter()
            .map(|(label, _)| format!("{label:?}"))
            .collect::<Vec<_>>();
        labels.sort();
        println!("All schedules: {}\n", labels.join(", "));
    }

    fn list_main_schedules(app: &mut App) {
        let main_labels = r!(app.world().get_resource::<MainScheduleOrder>())
            .labels
            .iter()
            .map(|label| format!("{label:?}"))
            .collect::<Vec<_>>();
        println!("Main schedules: {}\n", main_labels.join(", "));
    }

    fn list_systems(app: &mut App, label: impl ScheduleLabel + Clone) {
        // Get systems.
        let schedules = r!(app.world().get_resource::<Schedules>());
        let schedule = r!(schedules.get(label.clone()));
        let graph = schedule.graph();
        let mut systems = graph.systems().map(|(x, ..)| x).collect::<Vec<_>>();

        // Sort systems topologically by dependency graph.
        let mut system_order = graph
            .dependency()
            .cached_topsort()
            .iter()
            .filter(|&&node| graph.get_system_at(node).is_some());
        systems.sort_by_key(|x| system_order.position(|y| x == y).unwrap_or(usize::MAX));

        // Print systems.
        for system in systems {
            let system_name = c!(graph.get_system_at(system)).name();
            println!("[{label:?}] {system_name}");
        }
    }

    main()
}
