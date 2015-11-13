extern crate primal;



fn main() {
    let limit = 100_000_000;
    let sieve = primal::Sieve::new(limit);
    for number in 8..limit {
        find_quad(&sieve, number).unwrap();
        // if number % 100_000 == 0 {
        //     println!("Progress {}", number);
        // }
    }
}


fn find_quad(sieve: &primal::Sieve, target: usize) -> Option<(usize, usize, usize, usize)> {
    if target < 8 {
        return None;
    }

    for c in sieve.primes_from(0) {
        {
            let total = 4 + c;
            if total > target {
                break;
            }
            let d = target - total;
            if sieve.is_prime(d) {
                return Some((2, 2, c, d));
            }
        }
        {
            let total = 5 + c;
            if total > target {
                break;
            }
            let d = target - total;
            if sieve.is_prime(d) {
                return Some((2, 3, c, d));
            }
        }
    }
    None
}

#[cfg(test)]
extern crate rand;

#[cfg(test)]
use rand::distributions::{IndependentSample, Range};

#[test]
fn find_quad_4() {
    assert!(test_number(4).is_none());
}

#[test]
fn find_quad_8() {
    assert_eq!(test_number(8).unwrap(), (2, 2, 2, 2));
}

#[test]
fn find_quad_37() {
    assert_eq!(test_number(37).unwrap(), (2, 2, 2, 31));
}

#[test]
fn find_quad_random() {
    let between = Range::new(8usize, 10_000_000);
    let mut rng = rand::thread_rng();
    for _ in 0..100 {
        let input = between.ind_sample(&mut rng);
        test_number(input);
    }
}

#[cfg(test)]
fn test_number(input: usize) -> Option<(usize, usize, usize, usize)> {
    let sieve = primal::Sieve::new(10_000_000);
    println!("attempting {}", input);
    let answer = find_quad(&sieve, input);
    println!("got {:?}", answer);
    if input >= 8 {
        let quad = answer.unwrap();
        let total = quad.0 + quad.1 + quad.2 + quad.3;
        assert_eq!(total, input);
    } else {
        assert!(answer.is_none());
    }
    return answer
}
