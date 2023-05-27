mod deserializer;
mod point;

#[cfg(test)]
mod tests;

pub struct Mrg {
    pub easy: Difficulty,
    pub normal: Difficulty,
    pub hard: Difficulty,
}

pub struct Track {
    pub name: String,
    pub start: Point,
    pub finish: Point,
    pub points: Vec<Point>,
}

pub struct Difficulty {
    pub tracks: Vec<Track>,
}

pub struct Point {
    pub x: i32,
    pub y: i32,
}
