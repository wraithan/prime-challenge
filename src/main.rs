extern crate primal;
extern crate time;
extern crate scoped_threadpool;

use std::env;
use time::Duration;
use scoped_threadpool::Pool;

fn main() {
    let mut args = env::args();
    let limit = args.nth(1)
        .and_then(|arg| arg.parse::<f64>().ok().map(|x| x as usize))
        .unwrap_or(10_000_000);

    let threads = 8;
    let chunks = threads*2;
    let chunk_size = limit/chunks;

    let mut pool = Pool::new(threads as u32);
    let time = Duration::span(|| {
        pool.scoped(|scope| {
            for i in 0..chunks {
                let sieve = primal::Sieve::new(limit);
                scope.execute(move || {
                    for number in (i*chunk_size)..(i+1)*chunk_size {
                        let answer = find_quad(&sieve, number);
                        if number >= 8 {
                            answer.unwrap();
                        }
                    }
                })
            }
        })
    });

    let ns = time.num_nanoseconds().unwrap();
    let (s, ns) = (ns / 1_000_000_000, ns % 1_000_000_000);
    let formatted_time = format!("{}.{:06}", s, ns / 1000);
    println!("Computed the solutions for 0..{} in {}s", limit, formatted_time);
}


fn find_quad(sieve: &primal::Sieve, target: usize) -> Option<(usize, usize, usize, usize)> {
    if target < 8 {
        return None;
    }

    let b;
    let cd_total;
    if target % 2 == 0 {
        b = 2;
        cd_total = target - 4;
    } else {
        b = 3;
        cd_total = target - 5;
    }

    for c in sieve.primes_from(0) {
        let d = cd_total - c;
        if sieve.is_prime(d) {
            return Some((2, b, c, d));
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
    test_number(37).unwrap();
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
