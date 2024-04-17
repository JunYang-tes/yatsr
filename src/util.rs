pub fn linear_interpolation<S,T>(t: S, a: T, b: T) -> T
where
  T: Copy + std::ops::Sub<Output = T> + std::ops::Mul<S, Output = T> + std::ops::Add<Output = T>,
{
  a + (b - a) * t
}
