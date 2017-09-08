use std;

pub trait Semigroup {
    fn append(self, other: Self) -> Self;
}

impl Semigroup for () {
    fn append(self, (): Self) -> Self {
        ()
    }
}

impl<T> Semigroup for Option<T>
    where T: Semigroup
{
    fn append(self, other: Self) -> Self {
        match (self, other) {
            (None, None) => None,
            (Some(x), None) |
            (None, Some(x)) => Some(x),
            (Some(a), Some(b)) => Some(a.append(b)),
        }
    }
}

macro_rules! impl_semigroup_for_add {
    ($ty:ty) => (
        impl Semigroup for $ty {
            fn append(self, other: Self) -> Self {
                self + other
            }
        }
    )
}

impl_semigroup_for_add!(usize);
impl_semigroup_for_add!(u8);
impl_semigroup_for_add!(u16);
impl_semigroup_for_add!(u32);
impl_semigroup_for_add!(u64);

impl_semigroup_for_add!(isize);
impl_semigroup_for_add!(i8);
impl_semigroup_for_add!(i16);
impl_semigroup_for_add!(i32);
impl_semigroup_for_add!(i64);

impl_semigroup_for_add!(std::time::Duration);

#[cfg(test)]
mod tests {
    use Semigroup;

    #[test]
    fn option_is_semigroup() {
        assert_eq!(None::<i32>, None.append(None));
        assert_eq!(Some(123), Some(123).append(None));
        assert_eq!(Some(456), None.append(Some(456)));
        assert_eq!(Some(579), Some(123).append(Some(456)));
    }
}
