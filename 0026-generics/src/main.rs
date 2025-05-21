use std::any::{type_name, TypeId};
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Bounds {
    top_left: Point,
    bottom_right: Point,
}

static SCREEN_BOUNDS:Bounds = Bounds {
    top_left: Point { x: 0.0, y: 0.0 },
    bottom_right: Point { x: 100.0, y: 100.0 },
};
pub trait Draw{
    fn bounds(&self) -> Bounds;
}

#[derive(Clone)]
struct Square {
    top_left: Point,
    size: f64,
}
impl Draw for Square {
    fn bounds(&self) -> Bounds {
        Bounds {
            top_left: self.top_left,
            bottom_right: Point {
                x: self.top_left.x + self.size,
                y: self.top_left.y + self.size,
            },
        }
    }
}

#[derive(Clone, Debug)]
struct Circle {
    center: Point,
    radius: f64,
}

impl Draw for Circle {
    fn bounds(&self) -> Bounds {
        Bounds {
            top_left: Point {
                x: self.center.x - self.radius,
                y: self.center.y - self.radius,
            },
            bottom_right: Point {
                x: self.center.x + self.radius,
                y: self.center.y + self.radius,
            },
        }
    }
}


fn overlap(a: Bounds, b: Bounds) -> Option<Bounds> {
    let top_left = Point {
        x: a.top_left.x.max(b.top_left.x),
        y: a.top_left.y.max(b.top_left.y),
    };
    let bottom_right = Point {
        x: a.bottom_right.x.min(b.bottom_right.x),
        y: a.bottom_right.y.min(b.bottom_right.y),
    };
    if top_left.x < bottom_right.x && top_left.y < bottom_right.y {
        Some(Bounds {
            top_left,
            bottom_right,
        })
    } else {
        None
    }
}
pub fn on_screen<T: 'static + Draw>(draw: &T) -> bool {
    println!("TypeId: {:?} ", TypeId::of::<&T>());
    println!("Type of draw: {:?}", type_name::<T>());
    overlap(SCREEN_BOUNDS, draw.bounds()).is_some()
}

fn main() {
    let square = Square {
        top_left: Point { x: 0.0, y: 0.0 },
        size: 10.0,
    };

    let visible = on_screen(&square);
    println!("Square is visible: {}", visible);

    let square = Square {
        top_left: Point { x: 120.0, y: 0.0 },
        size: 10.0,
    };
    let visible = on_screen(&square);
    println!("Square2 is visible: {}", visible);

    let visible = on_screen(&Circle {
        center: Point { x: 50.0, y: 50.0 },
        radius: 10.0,
    });
    println!("Circle is visible: {}", visible);
}
