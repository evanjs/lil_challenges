fn unique<T: Ord>(mut nums: Vec<T>) -> Vec<T> {
    match nums.len() {
        0 => vec![],
        _ => {
            nums.sort();
            nums.dedup();
            nums
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use test_case::test_case;
    use super::unique;

    #[test_case(Vec::from([1i32,2i32,3i32,3i32]), Vec::from([1i32,2i32,3i32]))]
    #[test_case(vec![1,6,2,5,9,9,1], vec![1,2,5,6,9])]
    #[test_case(vec!["a","d","d", "b"], vec!["a","b","d"])]
    #[test_case(Vec::<u32>::new(), Vec::new())]
    fn check_unique<'a, T: 'a + Ord + Debug>(nums: Vec<T>, answer: Vec<T>) {
        let before: Vec<T> = nums.into();
        let unique_nums = unique(before);
        assert_eq!(unique_nums, answer, "Lists do not match. {unique_nums:#?} should be {answer:#?}");
    }
}
