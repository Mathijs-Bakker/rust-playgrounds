// People entering the bus and people exiting the bus:
fn main() {
    println!("Solution: {}", number(&[(10, 0), (3, 5), (5, 8)]));
}

fn number(bus_stops: &[(i32, i32)]) -> i32 {
    // let mut sum: i32 = 0;

    // for (a, b) in bus_stops {
    //     sum += a;
    //     sum -= b;
    // }
    // sum

    // A 'for' loop, which genererates a result (like above)
    // can be turned into a 'fold':
    bus_stops
        .iter()
        .fold(0, |acc, (entering, exiting)| acc + entering - exiting)

    // Another solution:
    // bus_stops.iter().map(|(a, b)| a - b).sum()
}

#[test]
fn returns_expected() {
    assert_eq!(number(&[(10, 0), (3, 5), (5, 8)]), 5);
    assert_eq!(
        number(&[(3, 0), (9, 1), (4, 10), (12, 2), (6, 1), (7, 10)]),
        17
    );
    assert_eq!(
        number(&[(3, 0), (9, 1), (4, 8), (12, 2), (6, 1), (7, 8)]),
        21
    );
}
