use std::cmp::{max, min, Ordering};
use std::ops::Range;

pub trait RangeCmp<T: PartialOrd> {
    fn range_cmp(&self, other: &Range<T>) -> Ordering;
}

impl RangeCmp<usize> for Range<usize> {
    fn range_cmp(&self, other: &Range<usize>) -> Ordering {
        (self.start, self.end)
            .partial_cmp(&(other.start, other.end))
            .unwrap()
    }
}

#[derive(Debug)]
pub enum Domain {
    Source(Range<usize>),
    Destination(Range<usize>),
}

#[derive(Debug)]
pub struct RangeOverlap {
    pub overlap: Range<usize>,
    pub less_remainder: Option<Domain>,
    pub greater_remainder: Option<Domain>,
}

impl RangeOverlap {
    pub fn new(lhs: &Range<usize>, rhs: &Range<usize>) -> Option<Self> {
        let overlapopt = range_intersection(lhs, rhs);
        match overlapopt {
            Some(overlap) => {
                let mut less_remainder: Option<Domain> = None;
                let mut greater_remainder: Option<Domain> = None;
                if lhs.start < overlap.start {
                    less_remainder = Some(Domain::Source(lhs.start..overlap.start));
                }
                if overlap.end < lhs.end {
                    greater_remainder = Some(Domain::Source(overlap.end..lhs.end));
                }
                if rhs.start < overlap.start {
                    less_remainder = Some(Domain::Destination(rhs.start..overlap.start));
                }
                if overlap.end < rhs.end {
                    greater_remainder = Some(Domain::Destination(overlap.end..rhs.end));
                }
                Some(RangeOverlap {
                    overlap,
                    less_remainder,
                    greater_remainder,
                })
            }
            None => None,
        }
    }
}

pub fn range_intersection<T: Clone + Ord>(r1: &Range<T>, r2: &Range<T>) -> Option<Range<T>> {
    let max_start = max(&r1.start, &r2.start);
    let min_end = min(&r1.end, &r2.end);

    if min_end > max_start {
        Some(max_start.clone()..min_end.clone())
    } else {
        None
    }
}
