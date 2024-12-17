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
mod door;
mod lift;

#[doc(hidden)]
pub use crate::{door::*, lift::*};

pub struct BevyInfrastructurePlugin;

impl Plugin for BevyInfrastructurePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(door::BevyDoorPlugin);
    }
}
