use crate::Canvas;

pub trait Draw {
    fn draw(&self, canvas: &Canvas);
}
