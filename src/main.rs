use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_scene_hook::{HookPlugin, HookedSceneBundle, SceneHook};

#[derive(Component, Debug)]
struct Cube;

enum Color {
    Blue,
    Red,
}

#[derive(Component)]
struct Light(Color);

#[derive(Component)]
struct Camera;

fn load_scene(mut cmds: Commands, asset_server: Res<AssetServer>) {
    cmds.spawn(HookedSceneBundle {
        scene: SceneBundle {
            scene: asset_server.load("green.glb#Scene0"),
            ..default()
        },
        hook: SceneHook::new(|entity, cmds| {
            match entity.get::<Name>().map(|t| t.as_str()) {
                Some("greencube") => cmds.insert(Cube),
                Some("bluelight") => cmds.insert(Light(Color::Blue)),
                Some("redlight") => cmds.insert(Light(Color::Red)),
                Some("camera") => cmds.insert(Camera),
                _ => cmds,
            };
        }),
    });
}

fn transform_cam(query: Query<&mut Transform, With<Camera>>) {
    for (i, c) in query.iter().enumerate() {
        println!("t_cam[{}]: {:?}", i, c);
    }
}

fn transform_lit(mut query: Query<&mut Transform, With<Light>>) {
    for (i, mut c) in query.iter_mut().enumerate() {
        if i > 1 {
            println!("???");
        }
        
        let e = Quat::from_euler(EulerRot::XYZ, 0.05, 0.05, 0.05);
        
        c.translate_around(Vec3::new(0., 0., 0.), e);
        
    }
}

fn transform_cub(mut cubes: Query<&mut Transform, With<Cube>>) {
    for (i, mut c) in cubes.iter_mut().enumerate() {
        if i > 0 {
            println!("???");
        }
        let axis = Vec3::new(0.5, 1., 2.);
        c.rotate_axis(axis,0.005);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "I am a window!".to_string(),
                width: 500.,
                height: 300.,
                ..default()
            },
            ..default()
        }))
        .add_plugin(HookPlugin)
        .add_plugin(WorldInspectorPlugin)
        // .insert_resource(AmbientLight {
        //     color: Color::WHITE,
        //     brightness: 1.0,
        // })
        .add_startup_system(load_scene)
        // .add_startup_system(setup)
        .add_system(transform_cam)
        .add_system(transform_lit)
        .add_system(transform_cub)
        .run();
}

// fn setup(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
// ) {
//     // Cube'n shit
//     commands.spawn(SceneBundle {
//         // transform: Transform::from_xyz(15.0, 15.0, 15.0).with_scale(Vec3::splat(10.0)),
//         scene: asset_server.load("green.glb#Scene0"),
//         ..default()
//     });
// }
