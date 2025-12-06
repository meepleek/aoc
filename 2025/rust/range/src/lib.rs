use std::ops::{Range, RangeInclusive};

pub fn merge_ranges<T: PartialOrd + Clone + Copy>(a: &Range<T>, b: &Range<T>) -> Option<Range<T>> {
    let (from, to) = merge_tuple_ranges((a.start, a.end), (b.start, b.end))?;
    Some(from..to)
}

pub fn merge_inclusive_ranges<T: PartialOrd + Clone + Copy>(
    a: &RangeInclusive<T>,
    b: &RangeInclusive<T>,
) -> Option<RangeInclusive<T>> {
    let (from, to) = merge_tuple_ranges((*a.start(), *a.end()), (*b.start(), *b.end()))?;
    Some(from..=to)
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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case((0, 1), (2, 3), None)]
    #[case((0, 1), (1, 3), Some((0, 3)))]
    #[case((0, 5), (1, 10), Some((0, 10)))]
    #[case((0, 1), (0, 3), Some((0, 3)))]
    #[case((0, 100), (10, 11), Some((0, 100)))]
    fn merge_ranges(
        #[case] a: (usize, usize),
        #[case] b: (usize, usize),
        #[case] expected_result: Option<(usize, usize)>,
    ) {
        let merged = merge_tuple_ranges(a, b);
        let merged_reverse = merge_tuple_ranges(b, a);

        assert_eq!(expected_result, merged);
        assert_eq!(expected_result, merged_reverse);
    }
}
