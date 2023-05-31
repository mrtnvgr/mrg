use crate::Mrg;
use std::fs;

#[test]
fn check_tracks_count() {
    let file = get_mrg();
    assert_eq!(file.easy.tracks.len(), 10);
    assert_eq!(file.normal.tracks.len(), 10);
    assert_eq!(file.hard.tracks.len(), 10);
}

#[test]
fn check_track() {
    let mrg = get_mrg();

    let track = &mrg.easy.tracks.first().unwrap();

    assert_eq!(track.name, "Intro");
    assert_eq!(track.start.to_tuple(), (-56, 24));
    assert_eq!(track.finish.to_tuple(), (432, 0));

    let points = &track.points;
    assert_eq!(points.len(), 45);

    assert_eq!(points.first().unwrap().to_tuple(), (-380, 136));
    assert_eq!(points.last().unwrap().to_tuple(), (798, 67));
}

fn get_mrg() -> Mrg {
    let bytes = fs::read("levels.mrg").unwrap();
    Mrg::from_bytes(bytes).unwrap()
}
