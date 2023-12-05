pub mod io;

use std::{
    cmp::{max, min},
    fmt::Debug,
};
pub fn ranges_overlap<T: Ord + Debug>(r1: (&T, &T), r2: (&T, &T)) -> bool {
    let start_max = max(r1.0, r2.0);
    let end_min = min(r1.1, r2.1);
    end_min > start_max
}
