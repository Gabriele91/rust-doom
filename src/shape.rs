use crate::math::{Vector2, Vector4};

pub trait Size<T> {
    fn width(&self) -> T;
    fn height(&self) -> T;
}

impl<T: Sized + Copy> Size<T> for Vector2<T> {
    fn width(&self) -> T {
        self.x
    }

    fn height(&self) -> T {
        self.y
    }
}

pub trait Rectangle<T> {
    fn top(&self) -> T;
    fn bottom(&self) -> T;
    fn left(&self) -> T;
    fn right(&self) -> T;
}

impl<T: Sized + Copy> Rectangle<T> for Vector4<T> {
    fn top(&self) -> T {
        self.x
    }

    fn bottom(&self) -> T {
        self.y
    }

    fn left(&self) -> T {
        self.z
    }

    fn right(&self) -> T {
        self.w    
    }
}