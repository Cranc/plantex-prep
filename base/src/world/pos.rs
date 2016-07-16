
use super::ChunkIndex;
use std::fmt;
use math::{self, Vec2f, Vec2i};


/// using odd-r layout
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct AxialPos {
    pub q: ChunkIndex,
    pub r: ChunkIndex,
}

impl AxialPos {
    pub fn new(q: ChunkIndex, r: ChunkIndex) -> Self {
        AxialPos { q: q, r: r }
    }

    pub fn to_real(&self) -> Vec2f {
        Vec2f {
            x: ((2 * self.q + self.r) as math::DefaultFloat) * super::HEX_INNER_RADIUS,
            y: (self.r as math::DefaultFloat) * (3.0 / 2.0) * super::HEX_OUTER_RADIUS,
        }
    }

    pub fn to_vec(&self) -> Vec2i {
        (*self).into()
    }

    pub fn s(&self) -> ChunkIndex {
        -self.q - self.r
    }

    pub fn range(from: Self, to: Self) -> AxialRange {
        AxialRange {
            start: from,
            end: to,
            curr: from,
        }
    }
}

impl Into<Vec2i> for AxialPos {
    fn into(self) -> Vec2i {
        Vec2i::new(self.q, self.r)
    }
}

impl From<Vec2i> for AxialPos {
    fn from(v: Vec2i) -> Self {
        AxialPos::new(v.x, v.y)
    }
}

impl fmt::Debug for AxialPos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.q)
            .field(&self.r)
            .finish()
    }
}

/// Two dimensional, exclusive range of axial coordinates.
/// See TODO for more information.
pub struct AxialRange {
    start: AxialPos,
    end: AxialPos,
    curr: AxialPos,
}

impl Iterator for AxialRange {
    type Item = AxialPos;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr.r == self.end.r {
            return None;
        }

        let out = self.curr;

        self.curr.q += 1;
        // wrap around of q
        if self.curr.q == self.end.q {
            self.curr.r += 1;
            self.curr.q = self.start.q;
        }

        Some(out)
    }
}

#[test]
fn test_axial_range() {
    let r = AxialRange {
        start: AxialPos::new(1, 1),
        end: AxialPos::new(3, 3),
        curr: AxialPos::new(1, 1),
    };

    assert_eq!(
        r.collect::<Vec<_>>(),
        vec![
            AxialPos::new(1, 1),
            AxialPos::new(2, 1),
            AxialPos::new(1, 2),
            AxialPos::new(2, 2)
        ]
    );
}
