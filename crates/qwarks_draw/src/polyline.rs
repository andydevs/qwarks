use crate::{Canvas, Draw};

pub struct Polyline {
    points: Vec<(f64, f64)>,
}

impl Polyline {
    pub fn new() -> Self {
        Self { points: Vec::default() }
    }

    pub fn from_points(points: &[(f64, f64)]) -> Self {
        Self {
            points: points.to_vec(),
        }
    }
}

impl Draw for Polyline {
    fn draw(&self, canvas: &Canvas) {
        let Some((x0, y0)) = self.points.get(0) else {
            return;
        };
        canvas.ctx().begin_path();
        canvas.ctx().move_to(*x0, *y0);
        if let Some(rest) = self.points.get(1..) {
            for (x, y) in rest {
                canvas.ctx().line_to(*x, *y);
            }
            canvas.ctx().stroke();
        }
    }
}
