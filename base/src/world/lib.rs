
mod chunk;
mod ground;
mod hex_pillar;
mod world;

pub use chunk::Chunk;
pub use ground::Ground;
pub use hex_pillar::HexPillar;
pub use world::World;

pub const HEX_INNER_RADIUS: f32 = 3.0;
pub const PILLAR_STEP_HEIGHT: f32 = 0.5;
pub const CHUNK_SIZE: usize = 16;
