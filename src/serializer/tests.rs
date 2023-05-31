use crate::Mrg;
use std::fs;

#[test]
fn serialize_mrg() {
    let bytes = fs::read("levels.mrg").unwrap();

    let mrg = Mrg::from_bytes(bytes).unwrap();
    let mrg_track = &mrg.easy.tracks.first().unwrap();

    let new_bytes = mrg.to_bytes().unwrap();

    fs::write("/home/user/test.mrg", &new_bytes).unwrap();

    let new = Mrg::from_bytes(new_bytes).unwrap();
    let new_track = &new.easy.tracks.first().unwrap();

    assert_eq!(new_track.start.to_tuple(), mrg_track.start.to_tuple());

    assert_eq!(
        new_track.points.first().unwrap().to_tuple(),
        mrg_track.points.first().unwrap().to_tuple()
    );

    assert_eq!(new_track.points.len(), mrg_track.points.len());

    // FIXME: serializing should be lossless
    // assert_eq!(mrg.to_bytes(), bytes, "Serializing failed");
}
