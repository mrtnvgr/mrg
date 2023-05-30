use super::reader::Reader;
use crate::{Point, Track};
use anyhow::Result;

impl Track {
    pub fn new(reader: &mut Reader, name: String, offset: usize) -> Result<Self> {
        let mut track_reader = reader.clone_from_offset(offset);

        assert_eq!(track_reader.read_byte()?, 0x33, "Invalid track start byte");

        let perform_magic = |i| (i >> 16) << 3;

        let start_x = perform_magic(track_reader.read_int()?);
        let start_y = perform_magic(track_reader.read_int()?);
        let finish_x = perform_magic(track_reader.read_int()?);
        let finish_y = perform_magic(track_reader.read_int()?);

        let start = Point::new(start_x, start_y);
        let finish = Point::new(finish_x, finish_y);

        let points_count = track_reader.read_short()?;

        let mut point_x = track_reader.read_int()?;
        let mut point_y = track_reader.read_int()?;

        let mut points: Vec<Point> = Vec::new();
        points.push(Point::new(point_x, point_y));

        for _ in 1..points_count {
            let x = track_reader.read_byte()?;

            if x == -1 {
                point_x = track_reader.read_int()?;
                point_y = track_reader.read_int().unwrap_or(0);
            } else {
                let y = track_reader.read_byte()?;
                point_x += i32::from(x);
                point_y += i32::from(y);
            }

            points.push(Point::new(point_x, point_y));
        }

        Ok(Self {
            name,
            start,
            finish,
            points,
        })
    }
}
