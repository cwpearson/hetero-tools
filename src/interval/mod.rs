use std::collections::HashMap;

pub trait Interval {
    fn lb(&self) -> f64;
    fn ub(&self) -> f64;

    fn contains(&self, p: f64) -> bool {
        if p >= self.lb() && p < self.ub() {
            return true;
        }
        return false;
    }

    fn overlaps(&self, other: &Interval) -> bool {
        // self.lb within other
        if self.contains(other.lb()) {
            return true;
        }
        if self.contains(other.ub()) {
            return true;
        }
        if other.contains(self.lb()) {
            return true;
        }
        if other.contains(self.ub()) {
            return true;
        }

        return false;
    }
}

struct EndPoint {
    pos: f64,
    dir: bool,
}

pub struct IntervalSet {
    map: HashMap<f64, EndPoint>,
}
