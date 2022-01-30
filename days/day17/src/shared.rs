use lazy_static::lazy_static;
use regex::Regex;
use std::thread::sleep;
lazy_static! {
    static ref RE: Regex =
        Regex::new(r"target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)").unwrap();
}

pub struct Probe {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,

    ax: i32,
    ay: i32,
}

pub struct Bounds {
    pub x: Range,
    pub y: Range,
}

pub struct Range {
    pub min: i32,
    pub max: i32,
}
///
///
///
///
impl Probe {
    pub fn new(vx: i32, vy: i32) -> Self {
        Self {
            x: 0,
            y: 0,
            vx,
            vy,
            ax: 1,
            ay: -1,
        }
    }

    ///
    ///
    ///
    pub fn step(&mut self) {
        self.x = self.x + self.vx;
        self.y = self.y + self.vy;

        if self.vx == 0 {
        } else if self.vx > 0 {
            self.vx = self.vx - self.ax;
        } else if self.vx < 0 {
            self.vx = self.vx + self.ax;
        }

        self.vy = self.vy + self.ay;
    }

    ///
    ///
    ///
    pub fn is_beyond_bounds(&self, bounds: &Bounds) -> bool {
        self.x > bounds.x.max || self.y < bounds.y.min
    }

    pub fn is_within_bounds(&self, bounds: &Bounds) -> bool {
        (self.x <= bounds.x.max && self.x >= bounds.x.min)
            && (self.y <= bounds.y.max && self.y >= bounds.y.min)
    }

    ///
    ///
    pub fn simulate(&mut self, bounds: &Bounds) -> bool {
        let mut ret = false;
        loop {
            self.step();
            if self.is_within_bounds(bounds) {
                ret = true;
                break;
            } else if self.is_beyond_bounds(bounds) {
                ret = false;
                break;
            }
        }
        ret
    }
}
///
///
///
///
pub fn parse(input: String) -> Bounds {
    let mut iter = RE.captures_iter(input.as_str());
    let cap = iter.next().unwrap();
    println!("{} ,{}", &cap[1], &cap[2]);

    let x_min = *&cap[1].parse::<i32>().ok().unwrap();
    let x_max = *&cap[2].parse::<i32>().ok().unwrap();
    let y_min = *&cap[3].parse::<i32>().ok().unwrap();
    let y_max = *&cap[4].parse::<i32>().ok().unwrap();

    println!("x_min {} = ", x_min);
    println!("x_max {} = ", x_max);
    println!("y_min {} = ", y_min);
    println!("y_max {} = ", y_max);

    Bounds {
        x: Range {
            min: x_min,
            max: x_max,
        },
        y: Range {
            min: y_min,
            max: y_max,
        },
    }
}
