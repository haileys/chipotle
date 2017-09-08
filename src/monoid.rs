use Semigroup;

pub trait Monoid : Semigroup {
    fn empty() -> Self;
}

impl<T> Monoid for T
    where T: Semigroup + Default
{
    fn empty() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use Monoid;

    #[test]
    fn option_is_monoid() {
        let none: Option<()> = Monoid::empty();
        assert_eq!(None, none);

        let zero: i32 = Monoid::empty();
        assert_eq!(0, zero);
    }
}
