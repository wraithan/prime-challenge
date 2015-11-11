extern crate primal;

#[cfg(test)]
extern crate rand;


fn main() {
    let sieve = primal::Sieve::new(10_000_000);
    for number in sieve.primes_from(0).take_while(|x| *x < 50) {
        println!("got num {}", number)
    }
}


fn find_quad(target: u64) -> Option<(u64, u64, u64, u64)> {
    if target < 8 {
        return None;
    }

    let mut primes: Vec<u64> = vec![];
    let sieve = primal::Sieve::new(10_000_000);
    for number in sieve.primes_from(0).take_while(|x| *x < target as usize) {
        primes.push(number as u64);
    }

    for a in primes.iter() {
        for b in primes.iter() {
            for c in primes.iter() {
                let total = *a + *b + *c;
                if total > target {
                    break;
                }
                let d = target - total;
                if sieve.is_prime(d as usize) {
                    return Some((*a, *b, *c, d));
                }
            }
        }
    }
    None
}

use rand::distributions::{IndependentSample, Range};

#[test]
fn find_quad_4() {
    assert!(find_quad(4).is_none())
}

#[test]
fn find_quad_8() {
    assert_eq!(find_quad(8).unwrap(), (2, 2, 2, 2))
}

#[test]
fn find_quad_37() {
    assert_eq!(find_quad(37).unwrap(), (2, 2, 2, 31))
}

#[test]
fn find_quad_random() {
    let between = Range::new(8u64, 10_000_000);
    let mut rng = rand::thread_rng();
    for _ in 0..100 {
        let input = between.ind_sample(&mut rng);
        test_number(input)
    }
}

#[cfg(test)]
fn test_number(input: u64) {
    println!("attempting {}", input);
    let answer = find_quad(input).unwrap();
    println!("got {:?}", answer);
    let total = answer.0 + answer.1 + answer.2 + answer.3;
    assert_eq!(total, input)
}
