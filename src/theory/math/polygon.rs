use crate::theory::math::{Angle, Vec2};
use crate::theory::utils::macros::make_deref;
use crate::theory::utils::number_ext::UsizeExt;

pub fn are_counter_clockwise(pts: &[Vec2]) -> bool {
    let n = pts.len();
    if n > 2 {
        let mut min_i = (0..n)
            .min_by(|i, j| pts[*i].y().total_cmp(&pts[*j].y()))
            .unwrap();
        while pts[min_i.add_rem(-1, n)].y() == pts[min_i].y() && min_i != 1 {
            min_i = min_i.add_rem(-1, n)
        }
        let p = pts[min_i.add_rem(0, n)];
        let next_p = pts[min_i.add_rem(1, n)];
        let last_p = pts[min_i.add_rem(-1, n)];
        let angle = Angle::from_points(last_p, p, next_p);

        angle.turns_normalised() >= 0.5
    } else {
        true
    }
}

#[derive(Default, Clone, Debug)]
pub struct Polygon(pub Vec<Vec2>);
impl Polygon {
    pub fn new(pts: Vec<Vec2>) -> Self {
        debug_assert!(are_counter_clockwise(&pts));
        Self(pts)
    }
    pub fn new_ccw(mut pts: Vec<Vec2>) -> Self {
        if !are_counter_clockwise(&pts) {
            pts.reverse();
        }
        Self(pts)
    }
    pub fn new_cw(mut pts: Vec<Vec2>) -> Self {
        if are_counter_clockwise(&pts) {
            pts.reverse();
        }
        Self(pts)
    }
    pub fn new_regular(n_pts: usize, radius: f32) -> Self {
        Self(
            (0..n_pts)
                .map(|i| Angle::from_turn(i as f32 / n_pts as f32).to_vec() * radius)
                .collect(),
        )
    }
    // pub fn contains_point(&self, pt: Vec2) -> bool {
    //     // ray casting algo
    //     let mut count = 0;
    //     for i in 0..self.0.len() {
    //         let p1 = self.0[i];
    //         let p2 = self.0[(i + 1) % self.len()];
    //         let ray = Ray {
    //             start: pt,
    //             end: pt + VecN([1., 0.]),
    //         };
    //         if (ray.intersect_segment(Segment { start: p1, end: p2 })) {
    //             count += 1;
    //         }
    //     }
    //     count % 2 == 1
    // }

    // pub fn intersect_segment(&self, segment: Segment<2>) -> bool {
    //     if self.contains_point(segment.start) || self.contains_point(segment.end) {
    //         return true;
    //     }
    //     for i in 0..self.0.len() {
    //         let p1 = self.0[i];
    //         let p2 = self.0[(i + 1) % self.len()];
    //         let seg = Segment { start: p1, end: p2 };
    //         if seg.intersect_segment(segment) {
    //             return true;
    //         }
    //     }
    //     false
    // }
}
make_deref!(Polygon, Vec<Vec2>);
