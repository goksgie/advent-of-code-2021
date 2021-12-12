use std::collections::VecDeque;

/// given a window size, compute the number of increases between consequtive windows. 
/// The window is slided by one regardless of its size. In other words, step size
/// is always one. 
pub fn compute_increase_with_window<'a, T: IntoIterator<Item = &'a u64>>(numbers: T, wsize: usize) -> u64 {
    // if the window size is zero, return directly
    if wsize == 0 {
        return 0;
    }

    let mut result = 0;
    let mut window = VecDeque::new();
    let mut iter = numbers.into_iter();

    // pre-populate the window with elements.
    for _ in 0..wsize {
        if let Some(val) = iter.next() {
            window.push_back(val);
        }
    }

    // validation check on the window size
    if window.len() != wsize {
        return 0;
    }

    // compute the sliding window increases
    while let Some(val) = iter.next() {
        let old_val = window.pop_front().unwrap();
        if val > old_val {
            result += 1;
        }
        window.push_back(val);
    }

    result
}


#[test]
fn test_compute_increase() {
    let test_data: Vec<(Vec<u64>, usize, u64)> = vec![
        (vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263], 3, 5),
        (vec![1, 1, 1, 2, 1, 1, 1], 3, 1)
    ]; 

    for (sample, wsize, result) in test_data {
        assert_eq!(compute_increase_with_window(&sample, wsize), result);
    }
}

#[test]
fn test_compute_increase_invalid_window() {
    assert_eq!(compute_increase_with_window(&vec![], 1), 0);
    assert_eq!(compute_increase_with_window(&vec![1, 2], 0), 0);
    assert_eq!(compute_increase_with_window(&vec![1, 2, 3], 4), 0);
}