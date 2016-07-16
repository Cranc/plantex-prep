use std::collections::HashMap as AssocMap;
use std::ops;
use super::{AxialPos, Chunk, HexPillar};
use math::{self, Vec2i};

pub struct World {
    chunks: AssocMap<Vec2i, Chunk>,
}

impl World {
    pub fn empty() -> Self {
        World {
            chunks: AssocMap::new(),
        }
    }
}

impl ops::Index<AxialPos> for World {
    type Output = HexPillar;

    fn index(&self, pos: AxialPos) -> &Self::Output {
        let chunk_pos = pos.to_vec() / (super::CHUNK_SIZE as i32);

        match self.chunks.get(&chunk_pos) {
            None => panic!("chunk {:?} is not loaded (position request {:?})",
                chunk_pos,
                pos,
            ),
            Some(chunk) => {
                let inner_pos: Vec2i = pos.to_vec() % (super::CHUNK_SIZE as i32);
                &chunk[inner_pos.into()]
            }
        }
    }
}
