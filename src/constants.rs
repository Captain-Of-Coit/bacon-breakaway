// How much Corn we should earn per run
// DISTANCE / CORN_DISTANCE_EARN_DIVIDER
pub const CORN_DISTANCE_EARN_DIVIDER: u32 = 20;

pub const INITIAL_CAMERA_SPEED: f32 = 200.0;

pub const MAX_ENERGY: f32 = 100.0;

// How fast we move relative to the amount of energy we have available
// pub const ENERGY_MOVE_SPEED_MULTIPLIER: f32 = 1.1;

pub const STARTING_CORN: u32 = 0;

pub const MINIMUM_CORN_EARN: u32 = 10;


pub const FINISH_DISTANCE: f32 = 10000.0;

pub const STARTING_STATE: crate::states::GameState = crate::states::GameState::MainMenu;