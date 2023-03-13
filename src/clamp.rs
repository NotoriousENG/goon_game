pub trait Clamp<T> {
    fn clamp(self, min: T, max: T) -> T;
}

impl<T> Clamp<T> for T
where
    T: PartialOrd + Copy,
{
    fn clamp(self, min: T, max: T) -> T {
        if self > max {
            max
        } else if self < min {
            min
        } else {
            self
        }
    }
}
