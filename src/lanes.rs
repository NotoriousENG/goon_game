use crate::clamp::Clamp;
use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Lane {
    Left = -1,
    Middle = 0,
    Right = 1,
}

#[derive(Component)]
pub struct LaneEntity {
    pub lane: Lane,
}

impl LaneEntity {
    pub fn change_lane(&mut self, direction: i32) {
        // set the lane to the clamp of the current lane + the input
        let next_lane = Clamp::clamp(
            self.lane as i32 + direction,
            Lane::Left as i32,
            Lane::Right as i32,
        );
        self.lane = match next_lane {
            0 => Lane::Middle,
            1 => Lane::Right,
            -1 => Lane::Left,
            _ => panic!("Invalid lane value"),
        };
    }
}

impl Default for LaneEntity {
    fn default() -> Self {
        Self { lane: Lane::Middle }
    }
}
