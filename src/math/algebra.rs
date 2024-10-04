use std::{
    marker::PhantomData,
    ops::{Add, Neg},
};

/// 0
pub trait Zero {
    fn zero() -> Self;
}
/// 1
pub trait One {
    fn one() -> Self;
}
macro_rules! int_zero_one {
    ($x:ty) => {
        impl Zero for $x {
            fn zero() -> $x {
                0
            }
        }
        impl One for $x {
            fn one() -> $x {
                1
            }
        }
    };
}
int_zero_one!(u8);
int_zero_one!(u16);
int_zero_one!(u32);
int_zero_one!(u64);
int_zero_one!(u128);
int_zero_one!(usize);
int_zero_one!(i8);
int_zero_one!(i16);
int_zero_one!(i32);
int_zero_one!(i64);
int_zero_one!(i128);
int_zero_one!(isize);
/// マグマ
pub trait Magma {
    type Set;
    fn op(a: Self::Set, b: Self::Set) -> Self::Set;
}
/// 単位元
pub trait Identity: Magma {
    fn id() -> Self::Set;
}
/// 逆元
pub trait Inverse: Magma {
    fn inv(a: Self::Set) -> Self::Set;
}
/// 結合法則
pub trait Associative: Magma {}
/// 交換法則
pub trait Commutative: Magma {}
/// 半群
pub trait SemiGroup: Magma + Associative {}
/// モノイド
pub trait Monoid: SemiGroup + Identity {}
/// 群
pub trait Group: Monoid + Inverse {}
/// アーベル群
pub trait Abel: Group + Commutative {}
/// 分配法則
pub trait Distributive<Add: Magma> {}
/// 環
pub trait Ring<Add: Abel>: SemiGroup {}
/// 体
pub trait Field<Add: Abel>: Ring<Add> + Group {}
/// 加算のアーベル群を生成する
pub struct AddOp<T> {
    _marker: PhantomData<T>,
}
impl<T> Magma for AddOp<T>
where
    T: Add<T, Output = T>,
{
    type Set = T;
    fn op(a: Self::Set, b: Self::Set) -> Self::Set {
        a + b
    }
}
impl<T> Identity for AddOp<T>
where
    Self: Magma,
    Self::Set: Zero,
{
    fn id() -> Self::Set {
        Self::Set::zero()
    }
}
impl<T> Inverse for AddOp<T>
where
    Self: Magma,
    Self::Set: Neg<Output = Self::Set>,
{
    fn inv(a: Self::Set) -> Self::Set {
        -a
    }
}
impl<T> Associative for AddOp<T> where Self: Magma {}
impl<T> Commutative for AddOp<T> where Self: Magma {}
impl<T> SemiGroup for AddOp<T> where Self: Magma + Associative {}
impl<T> Monoid for AddOp<T> where Self: SemiGroup + Identity {}
impl<T> Group for AddOp<T> where Self: Monoid + Inverse {}
impl<T> Abel for AddOp<T> where Self: Group + Commutative {}
