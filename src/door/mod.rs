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
use super::*;

/// A Bevy event for door actions.
#[derive(Event)]
pub struct DoorEvent {
    name: String,
    goal: DoorGoal,
}

impl DoorEvent {
    pub fn open(name: String) -> Self {
        return DoorEvent {
            name,
            goal: DoorGoal::Open,
        };
    }

    pub fn close(name: String) -> Self {
        return DoorEvent {
            name,
            goal: DoorGoal::Closed,
        };
    }
}

/// A component bundle for doors.
#[derive(Bundle, Default)]
pub struct DoorBundle {
    pub door_properties: DoorProperties,
    pub door_dimensions: DoorDimensions,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

/// A component to store door properties.
#[derive(Component, Default)]
pub struct DoorProperties {
    name: String,
    swing_value: f32,
    door_type: DoorType,
}

impl DoorProperties {
    /// Create a new door properties component.
    pub fn new(name: String, swing_value: f32, door_type: DoorType) -> Self {
        DoorProperties {
            name,
            swing_value,
            door_type,
        }
    }
}

/// A component to store door dimensions.
#[derive(Component, Default)]
pub struct DoorDimensions {
    length: f32,
    height: f32,
    thickness: f32,
}

impl DoorDimensions {
    /// Create a new door dimensions component.
    pub fn new(length: f32, height: f32, thickness: f32) -> Self {
        DoorDimensions {
            length,
            height,
            thickness,
        }
    }
}

/// A enum to describe the door type.
pub enum DoorType {
    SingleSliding,
    DoubleSliding,
    SingleSwinging,
    DoubleSwinging,
}

impl Default for DoorType {
    fn default() -> Self {
        DoorType::SingleSliding
    }
}

/// A component to store the door's current state.
#[derive(Component, PartialEq)]
pub enum DoorState {
    Open,
    Closed,
    Opening,
    Closing,
}

impl Default for DoorState {
    fn default() -> Self {
        DoorState::Closed
    }
}

impl PartialEq<DoorGoal> for DoorState {
    fn eq(&self, other: &DoorGoal) -> bool {
        match self {
            DoorState::Open => other == &DoorGoal::Open,
            DoorState::Closed => other == &DoorGoal::Closed,
            _ => false,
        }
    }
}

/// A component to store the door's goal state.
#[derive(Component, PartialEq)]
pub enum DoorGoal {
    Open,
    Closed,
}

impl Default for DoorGoal {
    fn default() -> Self {
        DoorGoal::Closed
    }
}

impl PartialEq<DoorState> for DoorGoal {
    fn eq(&self, other: &DoorState) -> bool {
        match self {
            DoorGoal::Open => other == &DoorState::Open,
            DoorGoal::Closed => other == &DoorState::Closed,
        }
    }
}

/// A Bevy plugin for doors.
pub struct BevyDoorPlugin;

impl Plugin for BevyDoorPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DoorEvent>();
        app.add_systems(Update, spawn_door);
        app.add_systems(Update, update_door_goal);
        app.add_systems(Update, update_door_movement);
    }
}

/// A system to spawn doors.
/// 
/// The condition for spawning doors is when the door properties are added.
fn spawn_door(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut queries: Query<(Entity, &DoorProperties, &mut Transform, &DoorDimensions), Added<DoorProperties>>,
) {
    // spawn a parent and a controller
    for (entity, properties, mut transform, dimensions) in queries.iter_mut() {
        
        match properties.door_type {
            DoorType::SingleSliding => {}
            DoorType::DoubleSliding => {}
            DoorType::SingleSwinging => {
                let door = commands
                .spawn(PbrBundle {
                    mesh: meshes.add(Cuboid::new(
                        dimensions.length,
                        dimensions.height,
                        dimensions.thickness,
                    )),
                    material: materials.add(Color::srgb_u8(124, 144, 255)),
                    transform: Transform::from_xyz(
                        dimensions.length / 2.0,
                        dimensions.height / 2.0,
                        0.0,
                    ),
                    ..default()
                })
                .id();
    
            let joint = commands
                .spawn(PbrBundle {
                    transform: *transform,
                    ..default()
                })
                .id();
    
            transform.rotation = Quat::from_xyzw(0.0, 0.0, 0.0, 1.0);
    
            // Parent the child to the joint
            commands.entity(joint).add_child(door);
            commands.entity(entity).add_child(joint);
            commands.entity(entity).insert(DoorState::default());
            commands.entity(entity).insert(DoorGoal::default());
            }
            DoorType::DoubleSwinging => {}
        }
    }
}

/// A system to update the door goal based on the door event.
fn update_door_goal(
    mut door_requests: EventReader<DoorEvent>,
    mut queries: Query<(&DoorProperties, &DoorState, &mut DoorGoal), With<DoorProperties>>,
) {
    for door_request in door_requests.read() {
        for (properties, state, mut goal) in queries.iter_mut() {
            if door_request.name != properties.name {
                continue;
            }

            match door_request.goal {
                DoorGoal::Open => {
                    // Open the door
                    if *state == DoorState::Closed {
                        log::info!("Opening door {}", properties.name);
                        *goal = DoorGoal::Open;
                    }
                }
                DoorGoal::Closed => {
                    // Close the door
                    if *state == DoorState::Open {
                        log::info!("Closing door {}", properties.name);
                        *goal = DoorGoal::Closed;
                    }
                }
            }
        }
    }
}

/// A system to update the door movement based on the door goal.
fn update_door_movement(
    mut queries: Query<
        (&DoorProperties, &mut Transform, &mut DoorState, &DoorGoal),
        With<DoorProperties>,
    >,
) {
    for (properties, mut transform, mut state, goal) in queries.iter_mut() {
        if *goal == *state {
            continue;
        }

        debug!("Moving door {}", properties.name);

        match properties.door_type {
            DoorType::SingleSliding => {}
            DoorType::DoubleSliding => {}
            DoorType::SingleSwinging => match goal {
                DoorGoal::Closed => {
                    if transform.rotation.to_euler(EulerRot::ZYX).1.abs() <= 0.02 {
                        transform.rotation = Quat::from_xyzw(0.0, 0.0, 0.0, 1.0);
                        *state = DoorState::Closed;
                    } else {
                        *state = DoorState::Closing;
                        transform.rotate(Quat::from_rotation_y(-0.01*properties.swing_value.signum()));
                    }
                }
                DoorGoal::Open => {
                    debug!(
                        "Moving door {:?}",
                        transform.rotation.to_euler(EulerRot::ZYX)
                    );
                    if transform.rotation.to_euler(EulerRot::ZYX).1.abs() >= properties.swing_value.abs() {
                        transform.rotation = Quat::from_rotation_y(properties.swing_value);
                        *state = DoorState::Open;
                    } else {
                        *state = DoorState::Opening;
                        transform.rotate(Quat::from_rotation_y(0.01*properties.swing_value.signum()));
                    }
                }
            },
            DoorType::DoubleSwinging => {}
        }
    }
}
