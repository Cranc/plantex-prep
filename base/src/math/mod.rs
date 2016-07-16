
mod vec2;

pub type DefaultFloat = f32;
pub type DefaultInt = i32;
pub type DefaultUnsigned = u32;

pub use self::vec2::Vec2;
pub type Vec2f = Vec2<DefaultFloat>;
pub type Vec2i = Vec2<DefaultInt>;
pub type Vec2u = Vec2<DefaultUnsigned>;

// A bunch of compile time constants
pub const SQRT_3: f32 = 1.73205080757;
