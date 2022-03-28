#[allow(dead_code)]
fn unique<T: Ord + Clone>(nums: Vec<T>) -> Vec<T> {
    match nums.len() {
        0 => vec![],
        _ => {
            let mut cnums: Vec<T> = nums.iter().cloned().collect();
            nums.iter().enumerate().for_each(|(pos,val)|{
                let last = nums.iter().rposition(|v| v == val);
                // is this the last instance of <val> in the list?
                // if not, remove the last instance
                if let Some(last_instance) = last {
                    if last_instance != pos {
                        cnums.remove(last_instance);
                    }
                }

                // if the element is not the final instance, ignore it
            });
            cnums
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use test_case::test_case;
    use super::unique;

    #[test_case(Vec::from([1i32,2i32,3i32,3i32]), Vec::from([1i32,2i32,3i32]))]
    #[test_case(vec![1,6,2,5,9,9,1], vec![1,6,2,5,9])]
    #[test_case(vec!["a","d","d", "b"], vec!["a","d","b"])]
    #[test_case(Vec::<u32>::new(), Vec::new())]
    fn check_unique<'a, T: 'a + Ord + Debug + Clone>(nums: Vec<T>, answer: Vec<T>) {
        let before: Vec<T> = nums.into();
        let unique_nums = unique(before);
        assert_eq!(unique_nums, answer, "Lists do not match. {unique_nums:#?} should be {answer:#?}");
    }
}
