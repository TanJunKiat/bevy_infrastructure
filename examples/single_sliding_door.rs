// =========================================================================
/*
 * Copyright (C) 2019 Tan Jun Kiat
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
*/
// =========================================================================
use bevy::prelude::*;
use bevy_egui::*;
use bevy_infrastructure::*;
use bevy_panorbit_camera::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(EguiPlugin)
        .add_plugins(BevyInfrastructurePlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, ui_element)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(4.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });

    // door
    commands.spawn(DoorBundle {
        door_properties: DoorProperties::new("door_1".to_string(), 1.0, DoorType::SingleSliding),
        door_dimensions: DoorDimensions::new(1.0, 2.0, 0.05),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(0.1, 0.1, 0.1)),
        material: materials.add(Color::BLACK),
        transform: Transform::from_xyz(1.0, 0.0, 0.0),
        ..default()
    });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // camera
    commands.spawn((
        PanOrbitCamera::default(),
        Camera3dBundle {
            transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
    ));
}

fn ui_element(mut contexts: EguiContexts, mut door_request: EventWriter<DoorEvent>) {
    egui::Window::new("Test window").show(contexts.ctx_mut(), |ui| {
        if ui.button("Open door").clicked() {
            door_request.send(DoorEvent::open("door_1".to_string()));
        }
        if ui.button("Close door").clicked() {
            door_request.send(DoorEvent::close("door_1".to_string()));
        }
    });
}
