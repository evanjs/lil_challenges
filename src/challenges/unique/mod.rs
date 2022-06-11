use std::hash::Hash;
use std::fmt::Debug;

#[allow(dead_code)]
fn unique<T: Ord + Clone + Hash + Debug>(nums: Vec<T>) -> Vec<T> {
    match nums.len() {
        0 => vec![],
        _ => {
            let mut fin_coll: Vec<T> = Vec::new();
            nums.iter().for_each(|val| {
                let owned = val.to_owned();
                if fin_coll.contains(val) {
                    return
                }
                fin_coll.push(owned);
            }); 
            fin_coll
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;
    use std::hash::Hash;

    use test_case::test_case;
    use super::unique;

    #[test_case(Vec::from([1i32,2i32,3i32,3i32]), Vec::from([1i32,2i32,3i32]))]
    #[test_case(vec![1,6,2,5,9,9,1], vec![1,6,2,5,9])]
    #[test_case(vec!["a","d","d", "b"], vec!["a","d","b"])]
    #[test_case(Vec::<u32>::new(), Vec::new())]
    fn check_unique<'a, T: 'a + Ord + Debug + Clone + Hash>(nums: Vec<T>, answer: Vec<T>) {
        let before: Vec<T> = nums.into();
        let unique_nums = unique(before);
        assert_eq!(unique_nums, answer, "Lists do not match. {unique_nums:#?} should be {answer:#?}");
    }
}
