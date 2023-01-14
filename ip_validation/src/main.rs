// Write an algorithm that will identify valid IPv4 addresses in dot-decimal
// format. IPs should be considered valid if they consist of four octets,
// with values between 0 and 255, inclusive.
fn main() {
    println!("Solution {:?}", is_valid_ip("012.255.56.1"));
}

fn is_valid_ip(ip: &str) -> bool {
    let tmp: Vec<&str> = ip.split('.').collect();

    if tmp.len() != 4 {
        return false;
    }

    let mut is_valid = false;

    for s in tmp {
        if s.len() > 1 && s.starts_with('0') {
            is_valid = false;
            break;
        }

        let n = s.parse::<u8>();

        match n {
            Ok(_) => is_valid = true,
            Err(_) => is_valid = false,
        };

        if !is_valid {
            break;
        }
    }

    is_valid
}

#[cfg(test)]
mod tests {
    use super::is_valid_ip;

    #[test]
    fn sample_test() {
        assert!(is_valid_ip("0.0.0.0"));
        assert!(is_valid_ip("12.255.56.1"));
        assert!(is_valid_ip("137.255.156.100"));

        assert!(!is_valid_ip(""));
        assert!(!is_valid_ip("abc.def.ghi.jkl"));
        assert!(!is_valid_ip("123.456.789.0"));
        assert!(!is_valid_ip("12.34.56"));
        assert!(!is_valid_ip("01.02.03.04"));
        assert!(!is_valid_ip("256.1.2.3"));
        assert!(!is_valid_ip("1.2.3.4.5"));
        assert!(!is_valid_ip("123,45,67,89"));
        assert!(!is_valid_ip("1e0.1e1.1e2.2e2"));
        assert!(!is_valid_ip(" 1.2.3.4"));
        assert!(!is_valid_ip("1.2.3.4 "));
        assert!(!is_valid_ip("12.34.56.-7"));
        assert!(!is_valid_ip("1.2.3.4\n"));
        assert!(!is_valid_ip("\n1.2.3.4"));
        assert!(!is_valid_ip("174.00.170.0"));
    }
}
