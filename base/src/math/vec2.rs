use num_traits::Zero;
use std::ops;
use std::fmt;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Vec2 { x: x, y: y }
    }

    pub fn zero() -> Self
        where T: Zero
    {
        Vec2 {
            x: T::zero(),
            y: T::zero(),
        }
    }

    pub fn convert<U>(self) -> Vec2<U>
        where U: From<T> + Clone
    {
        Vec2::new(self.x.into(), self.y.into())
    }
}

impl<T> fmt::Debug for Vec2<T>
    where T: fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.x)
            .field(&self.y)
            .finish()
    }
}

macro_rules! impl_scalar_ops {
    ($op_trait:ident, $op_fn:ident) => {
        impl<T, U> ops::$op_trait<U> for Vec2<T>
            where T: ops::$op_trait<U>,
                  U: Clone
        {
            type Output = Vec2<<T as ops::$op_trait<U>>::Output>;
            fn $op_fn(self, rhs: U) -> Self::Output {
                Vec2::new(
                    ops::$op_trait::$op_fn(self.x, rhs.clone()),
                    ops::$op_trait::$op_fn(self.y, rhs),
                )
            }
        }
    }
}

impl_scalar_ops!(Mul, mul);
impl_scalar_ops!(Div, div);
impl_scalar_ops!(Rem, rem);

macro_rules! impl_vec_ops {
    ($op_trait:ident, $op_fn:ident) => {
        impl<T, U> ops::$op_trait<Vec2<U>> for Vec2<T>
            where T: ops::$op_trait<U>,
                  U: Clone
        {
            type Output = Vec2<<T as ops::$op_trait<U>>::Output>;
            fn $op_fn(self, rhs: Vec2<U>) -> Self::Output {
                Vec2::new(
                    ops::$op_trait::$op_fn(self.x, rhs.x),
                    ops::$op_trait::$op_fn(self.y, rhs.y),
                )
            }
        }
    }
}

impl_vec_ops!(Add, add);
impl_vec_ops!(Sub, sub);

#[test]
fn scalar_ops() {
    assert_eq!(Vec2::new(3, 2) * 2, Vec2::new(6, 4));
    assert_eq!(Vec2::new(3, 2) / 2, Vec2::new(1, 1));
    assert_eq!(Vec2::new(3, 2) % 2, Vec2::new(1, 0));
}

#[test]
fn vec_ops() {
    assert_eq!(Vec2::new(3, 3) + Vec2::new(2, -4), Vec2::new(5, -1));
    assert_eq!(Vec2::new(0, 2) - Vec2::new(2, -4), Vec2::new(-2, 6));
}
