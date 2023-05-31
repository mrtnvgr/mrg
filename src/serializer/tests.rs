use crate::Mrg;
use std::{env::temp_dir, fs, iter::zip};

#[test]
fn serialize_mrg() {
    let bytes = fs::read("levels.mrg").unwrap();
    let old = Mrg::from_bytes(bytes).unwrap();

    let new_bytes = old.to_bytes().unwrap();
    fs::write(temp_dir().join("test.mrg"), &new_bytes).unwrap();

    let new = Mrg::from_bytes(new_bytes).unwrap();

    for (old_diff, new_diff) in zip(old.iter_diffs(), new.iter_diffs()) {
        for (old_track, new_track) in zip(&old_diff.tracks, &new_diff.tracks) {
            assert_eq!(old_track.start.to_tuple(), new_track.start.to_tuple());
            assert_eq!(old_track.finish.to_tuple(), new_track.finish.to_tuple());

            assert_eq!(old_track.points.len(), new_track.points.len());

            for (old_point, new_point) in zip(&old_track.points, &new_track.points) {
                assert_eq!(old_point.to_tuple(), new_point.to_tuple());
            }
        }
    }

    // FIXME: serializing should be lossless
    // assert_eq!(mrg.to_bytes(), bytes, "Serializing failed");
}
