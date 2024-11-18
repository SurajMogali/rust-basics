
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}