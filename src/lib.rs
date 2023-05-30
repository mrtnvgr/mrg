mod deserializer;
mod point;
mod serializer;

#[derive(Clone)]
pub struct Mrg {
    pub easy: Difficulty,
    pub normal: Difficulty,
    pub hard: Difficulty,
}

#[derive(Clone)]
pub struct Track {
    pub name: String,
    pub start: Point,
    pub finish: Point,
    pub points: Vec<Point>,
}

#[derive(Clone)]
pub struct Difficulty {
    pub tracks: Vec<Track>,
}

#[derive(Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
