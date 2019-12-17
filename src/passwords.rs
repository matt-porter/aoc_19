use std::str::FromStr;

fn get_digits(number: i32) -> Vec<i32> {
    let str_password = format!("{}", &number);
    str_password.chars().map(|c| c.to_digit(10).unwrap() as i32).collect()
}

fn valid_password(password: i32) -> bool {
    let digits = get_digits(password);
    if digits.len() != 6 {return false;}
    let mut last = digits[0];
    let mut duplicate_exists = false;
    for d in digits[1..].iter() {
        if *d < last {
            return false; // not increasing
        }
        if *d == last {
            duplicate_exists = true;
        }
        last = *d;
    }
    duplicate_exists
}

fn valid_password_2(password: i32) -> bool {
    let digits = get_digits(password);
    if digits.len() != 6 {return false;}
    let mut duplicate_exists = false;
    for (i, d) in digits.iter().enumerate() {
        if i > 0 && *d < digits[i - 1] {
            return false; // not increasing
        }
    }
    for (i, _d) in digits.iter().enumerate() {
        let start: usize = (0i32.max((i as i32) - 2)) as usize ;
        let end: usize = (digits.len()-1).min(i+1);

        duplicate_exists = match digits[start..=end] {
            [a,b,c, d] => b == c && b != a && b!=d,
            [a, b, c] => if i == 1 {a==b && b!=c} else if i == 5 {b==c && b!=a} else {duplicate_exists},
            [_, _] => duplicate_exists,
            _ => {println!("Missed case? {} {:?}", i, &digits); duplicate_exists}
        };
        if duplicate_exists {
            break
        }
    }
    duplicate_exists
}

pub fn day4() {
    let input = "128392-643281";
    let mut s = input.split('-');
    let from = s.next().unwrap();
    let to = s.next().unwrap();
    let from = i32::from_str(from).unwrap();
    let to = i32::from_str(to).unwrap();
    println!("From {} to {}", from, to);
    let mut counter = 0;
    for password in from..=to {
        if valid_password(password) {
            counter += 1;
        }
    }
    println!("Day 4 p1 Count: {}", &counter);
    counter = 0;
    for password in from..=to {
        if valid_password_2(password) {
            counter += 1;
        }
    }
    println!("Day 4 p2 Count: {}", &counter);
}


#[cfg(test)]
mod day4 {
    use super::*;

    #[test]
    fn test_0() {
        assert!(valid_password(111111));
    }

    #[test]
    fn test_1() {
        assert!(!valid_password(223450));
    }

    #[test]
    fn test_2() {
        assert!(!valid_password(123789));
    }

    #[test]
    fn test_3() {
        assert!(valid_password(123444));
    }

    #[test]
    fn test_4() {
        // fail, no *just* pairs
        assert!(!valid_password_2(111111));
    }

    #[test]
    fn test_5() {
        assert!(!valid_password_2(223450));
    }

    #[test]
    fn test_6() {
        assert!(!valid_password_2(123789));
    }

    #[test]
    fn test_7() {
        assert!(!valid_password_2(123444));
    }

    #[test]
    fn test_8() {
        assert!(valid_password_2(112233));
    }

    #[test]
    fn test_9() {
        assert!(!valid_password_2(123444));
    }

    #[test]
    fn test_10() {
        assert!(valid_password_2(111122));
    }
    #[test]
    fn test_12() {
        assert!(valid_password_2(112222));
    }
    #[test]
    fn test_11() {
        assert!(!valid_password_2(222111));
    }
}