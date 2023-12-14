/*
 * StoryMaker - Living world generation tool
 * Copyright © 2022-2023 Nazim Lachter
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use bevy::prelude::*;
use big_brain::prelude::*;

pub mod drink;
pub mod move_to;
pub mod talk;
pub mod wait;
pub mod wander;

pub fn build(app: &mut App) {
  app.add_systems(
    PreUpdate,
    (
      drink::action,
      move_to::water::action,
      //talk::action,
      wait::action,
      wander::action,
    )
      .in_set(BigBrainSet::Actions),
  );
}
