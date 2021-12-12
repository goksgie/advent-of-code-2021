/// Compute the consequtive increase in the input and output the result
/// as u64.
pub fn compute_consequtive_increase<'a, T: IntoIterator<Item = &'a u64>>(numbers: T) -> u64 {
    let mut prev_elem = None;
    let mut result    = 0;

    for elem in numbers {
        if let Some(prev) = prev_elem {
            if elem > prev {
                result += 1;
            }
        }

        prev_elem = Some(elem);
    }
    
    result
}

#[test]
fn test_compute_increase() {
    let test_data: Vec<(Vec<u64>, u64)> = vec![
        (vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263], 7),
        (vec![1, 1, 1, 2, 1, 1, 1], 1)
    ];

    // the following iteration will consume the test data vector.
    for (test_inp, result) in test_data.iter() {
        assert_eq!(compute_consequtive_increase(test_inp), *result);        
    }
}