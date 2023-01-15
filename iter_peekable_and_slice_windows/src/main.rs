// Find all the elements of an array that are non-consecutive. A number is
// non-consecutive if it is not exactly one larger than the previous element
// in the array. The first element gets a pass and is never considered
// non-consecutive.
//
// E.g., if we have an array [1,2,3,4,6,7,8,15,16] then 6 and 15 are non-consecutive.

// Return the results as an array of tuples with two values:
// the index of the non-consecutive number and the non-consecutive number

fn main() {
    println!(
        "Solution: {:?} -- {:?}",
        find_all_non_consecutive_numbers_peekable(&[1, 2, 3, 4, 6, 7, 8, 10]),
        find_all_non_consecutive_numbers_windows(&[1, 2, 3, 4, 6, 7, 8, 10])
    );
}

fn find_all_non_consecutive_numbers_peekable(arr: &[i32]) -> Vec<(usize, i32)> {
    let mut res: Vec<(usize, i32)> = Vec::default();

    let mut p = arr.iter().peekable();
    p.next();

    for (idx, n) in arr.iter().enumerate() {
        match p.next() {
            Some(i) => {
                if *i == n + 1 {
                } else {
                    res.push((idx + 1, *i));
                }
            }
            None => break,
        }
    }

    res
}

fn find_all_non_consecutive_numbers_windows(arr: &[i32]) -> Vec<(usize, i32)> {
    arr.windows(2)
        .enumerate()
        .filter_map(|(idx, num)| {
            if num[0] + 1 == num[1] {
                None
            } else {
                Some((idx + 1, num[1]))
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peekable() {
        let expect: Vec<(usize, i32)> = vec![(4, 6), (7, 10)];
        let result = find_all_non_consecutive_numbers_peekable(&[1, 2, 3, 4, 6, 7, 8, 10]);

        assert_eq!(expect, result);
    }

    #[test]
    fn test_windows() {
        let expect: Vec<(usize, i32)> = vec![(4, 6), (7, 10)];
        let result = find_all_non_consecutive_numbers_windows(&[1, 2, 3, 4, 6, 7, 8, 10]);

        assert_eq!(expect, result);
    }
}
