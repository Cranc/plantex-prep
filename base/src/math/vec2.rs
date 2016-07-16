use num_traits::Zero;
use std::ops;
use std::fmt;

#[derive(PartialEq, Eq, Hash)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

// impl<T: Copy> ::std::marker::Copy for Vec2<T> {}
// impl<T: Clone> ::std::clone::Clone for Vec2<T> {}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Vec2 {
            x: x,
            y: y,
        }
    }

    pub fn zero() -> Self where T: Zero {
        Vec2 {
            x: T::zero(),
            y: T::zero(),
        }
    }

    pub fn convert<U>(self) -> Vec2<U> where U: From<T> + Clone {
        Vec2::new(self.x.into(), self.y.into())
    }
}

impl<T> fmt::Debug for Vec2<T> where T: fmt::Debug {
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
