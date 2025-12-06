use std::ops::{Range, RangeInclusive};

pub trait BoundedRange<T> {
    fn from(&self) -> T;
    fn to(&self) -> T;
    fn new(from: T, to: T) -> Self;
}

impl<T: Copy> BoundedRange<T> for Range<T> {
    fn from(&self) -> T {
        self.start
    }
    fn to(&self) -> T {
        self.end
    }
    fn new(from: T, to: T) -> Self {
        from..to
    }
}
impl<T: Copy> BoundedRange<T> for RangeInclusive<T> {
    fn from(&self) -> T {
        *self.start()
    }
    fn to(&self) -> T {
        *self.end()
    }
    fn new(from: T, to: T) -> Self {
        from..=to
    }
}
impl<T: Copy> BoundedRange<T> for (T, T) {
    fn from(&self) -> T {
        self.0
    }
    fn to(&self) -> T {
        self.1
    }
    fn new(from: T, to: T) -> Self {
        (from, to)
    }
}

pub fn merge_bounded_ranges<TElement: PartialOrd, TRange: BoundedRange<TElement>>(
    a: TRange,
    b: TRange,
) -> Option<TRange> {
    match true {
        _ if a.from() <= b.from() && a.to() >= b.to() => Some(TRange::new(a.from(), a.to())),
        _ if b.from() <= a.from() && b.to() >= a.to() => Some(TRange::new(b.from(), b.to())),
        _ if a.to() >= b.from() && a.to() <= b.to() => Some(TRange::new(a.from(), b.to())),
        _ if b.to() >= a.from() && b.to() <= a.to() => Some(TRange::new(b.from(), a.to())),
        _ => None,
    }
}

pub fn merge_tuple_ranges<T: PartialOrd + Clone + Copy>(
    (a_from, a_to): (T, T),
    (b_from, b_to): (T, T),
) -> Option<(T, T)> {
    match true {
        _ if a_from <= b_from && a_to >= b_to => Some((a_from, a_to)),
        _ if b_from <= a_from && b_to >= a_to => Some((b_from, b_to)),
        _ if a_to >= b_from && a_to <= b_to => Some((a_from, b_to)),
        _ if b_to >= a_from && b_to <= a_to => Some((b_from, a_to)),
        _ => None,
    }
}

pub fn merge_all_bounded_ranges<TElement: PartialOrd, TBound: BoundedRange<TElement> + Clone>(
    ranges: &mut Vec<TBound>,
) {
    'merge_ranges: loop {
        let ranges_range = 0..ranges.len();
        for i in ranges_range.clone() {
            for j in ranges_range.clone() {
                if i == j {
                    continue;
                }
                if let Some(merged_range) =
                    merge_bounded_ranges(ranges[i].clone(), ranges[j].clone())
                {
                    // remove from end
                    ranges.remove(i.max(j));
                    ranges.remove(j.min(i));
                    ranges.push(merged_range);
                    continue 'merge_ranges;
                }
            }
        }

        break;
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case((0, 1), (2, 3), None)]
    #[case((0, 1), (1, 3), Some((0, 3)))]
    #[case((0, 5), (1, 10), Some((0, 10)))]
    #[case((0, 1), (0, 3), Some((0, 3)))]
    #[case((0, 100), (10, 11), Some((0, 100)))]
    fn merge_bounded_ranges(
        #[case] a: (usize, usize),
        #[case] b: (usize, usize),
        #[case] expected_result: Option<(usize, usize)>,
    ) {
        let merged = super::merge_bounded_ranges(a, b);
        let merged_reverse = super::merge_bounded_ranges(b, a);

        assert_eq!(expected_result, merged);
        assert_eq!(expected_result, merged_reverse);
    }
}
