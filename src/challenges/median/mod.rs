use log::debug;

#[allow(dead_code)]
fn get_median(nums: &mut Vec<f32>) -> f32 {
    let len = nums.len();
    nums.sort_by(|a, b| a.partial_cmp(b).unwrap());
    match len % 2 {
        // even
        0 => {
            let one = nums.get(len / 2).unwrap();
            debug!("len / 2 :: {len} / 2 :: {one}");

            let two = nums.get(len / 2).unwrap() - 1f32;
            debug!("len / 2 + 1 :: {len} / 2 - 1 :: {two}");

            let result = (one + two) / 2f32;
            debug!("(one + two) / 2f32 :: ({one} + {two}) / 2f32 :: {result}");
            result
        }
        // odd
        _ => {
            let result = *nums.get(len / 2).unwrap();
            debug!("len / 2 + 1 :: {len} / 2 + 1 :: {result}");
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use crate::challenges::median::get_median;

    #[test_case(vec! [1f32, 2f32, 3f32], 2f32)]
    #[test_case(vec! [1f32, 2f32, 3f32, 4f32], 2.5f32)]
    #[test_case(vec![1f32, 2f32, 3f32, 5f32], 2.5f32)]
    #[test_case(vec![1f32, 2f32, 3f32, 5f32], 3.0f32 => panics "2.5 is not 3")]
    fn test_median(mut nums: Vec<f32>, answer: f32) {
        let result = get_median(&mut nums);
        println!("{result} should be {answer}");
        assert_eq!(result, answer, "{result} is not {answer}");
    }
}