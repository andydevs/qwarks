use crate::{Canvas, Draw};

pub struct Style {
    stroke: String,
    line_width: f64,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            stroke: String::from("black"),
            line_width: 1.0,
        }
    }
}

pub trait Shape: Draw + Sized {
    fn into_styled(self) -> Styled<Self> {
        Styled::new(self)
    }
}

pub struct Styled<S: Shape> {
    shape: S,
    style: Style,
}

impl<D: Shape> Styled<D> {
    fn new(shape: D) -> Self {
        Self {
            shape,
            style: Style::default(),
        }
    }

    pub fn stroke(mut self, stroke_style: &str) -> Self {
        self.style.stroke = String::from(stroke_style);
        self
    }

    pub fn line_width(mut self, stroke_width: f64) -> Self {
        self.style.line_width = stroke_width;
        self
    }
}

impl<S: Shape> Draw for Styled<S> {
    fn draw(&self, canvas: &Canvas) {
        canvas.ctx().save();
        canvas.ctx().set_stroke_style_str(&self.style.stroke);
        canvas.ctx().set_line_width(self.style.line_width);
        canvas.draw(&self.shape);
        canvas.ctx().restore();
    }
}
