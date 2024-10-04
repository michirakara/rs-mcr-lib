/*pub trait ModPow<T>
where
    Self: AddIdentity + Mul<Output = Self> + Rem<T, Output = Self> + Copy,
    T: Copy,
{
    fn modpow(&self, mut exp: u128, modulo: T) -> Self {
        let mut acc = Self::id();
        let mut base = *self;
        while exp > 0 {
            if exp & 1 == 1 {
                acc = acc * base % modulo;
            }
            base = base * base % modulo;
            exp >>= 1;
        }
        acc
    }
}

impl ModPow<u8> for u8 {}
impl ModPow<u16> for u16 {}
impl ModPow<u32> for u32 {}
impl ModPow<u64> for u64 {}
impl ModPow<u128> for u128 {}
impl ModPow<usize> for usize {}

impl ModPow<i8> for i8 {}
impl ModPow<i16> for i16 {}
impl ModPow<i32> for i32 {}
impl ModPow<i64> for i64 {}
impl ModPow<i128> for i128 {}
impl ModPow<isize> for isize {}
*/
