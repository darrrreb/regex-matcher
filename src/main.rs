use Regex::*;
use std::time::Instant;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Regex {
    Zero,
    One,
    Char(char),
    Alt(Box<Regex>, Box<Regex>),
    Seq(Box<Regex>, Box<Regex>),
    Star(Box<Regex>),
    NTimes(Box<Regex>, u32),
}

fn nullable(r: &Regex) -> bool {
    match r {
        Zero => false,
        One => true,
        Char(_) => false,
        Alt(r1, r2) => nullable(r1) || nullable(r2),
        Seq(r1, r2) => nullable(r1) && nullable(r2),
        Star(_) => true,
        NTimes(_, 0) => true,
        NTimes(r1, _) => nullable(r1),
    }
}

fn derivative(c: char, r: &Regex) -> Regex {
    match r {
        Zero => Zero,
        One => Zero,
        Char(d) => {
            if c == *d {
                One
            } else {
                Zero
            }
        }
        Alt(r1, r2) => Alt(Box::new(derivative(c, r1)), Box::new(derivative(c, r2))),
        Seq(r1, r2) => {
            if nullable(r1) {
                Alt(
                    Box::new(Seq(Box::new(derivative(c, r1)), r2.clone())),
                    Box::new(derivative(c, r2)),
                )
            } else {
                Seq(Box::new(derivative(c, r1)), r2.clone())
            }
        }
        Star(r1) => Seq(Box::new(derivative(c, r1)), Box::new(Star(r1.clone()))),
        NTimes(_, 0) => Zero,
        NTimes(r1, n) => Seq(Box::new(derivative(c, r1)), Box::new(NTimes(r1.clone(), n - 1))),
    }
}

fn simplify(r: Regex) -> Regex {
    match r {
        Alt(r1, r2) => match (simplify(*r1), simplify(*r2)) {
            (Zero, r2s) => r2s,
            (r1s, Zero) => r1s,
            (r1s, r2s) if r1s == r2s => r1s,
            (r1s, r2s) => Alt(Box::new(r1s), Box::new(r2s)),
        },
        Seq(r1, r2) => match (simplify(*r1), simplify(*r2)) {
            (Zero, _) => Zero,
            (_, Zero) => Zero,
            (One, r2s) => r2s,
            (r1s, One) => r1s,
            (r1s, r2s) => Seq(Box::new(r1s), Box::new(r2s)),
        },
        r => r,
    }
}

fn matcher(r: Regex, s: &str) -> bool {
    let result = s.chars().fold(r, |r, c| simplify(derivative(c, &r)));
    nullable(&result)
}

// (a?){n} a{n}
fn evil1(n: u32) -> Regex {
    Seq(
        Box::new(NTimes(Box::new(Alt(Box::new(Char('a')), Box::new(One))), n)),
        Box::new(NTimes(Box::new(Char('a')), n)),
    )
}

// (a*)* b
fn evil2() -> Regex {
    Seq(
        Box::new(Star(Box::new(Star(Box::new(Char('a')))))),
        Box::new(Char('b')),
    )
}

fn main() {
    // (a|b)* c
    let r = Seq(
        Box::new(Star(Box::new(Alt(
            Box::new(Char('a')),
            Box::new(Char('b')),
        )))),
        Box::new(Char('c')),
    );

    for s in ["c", "abbac", "abba", "abcab"] {
        println!("{s:>6}: {}", matcher(r.clone(), s));
    }

    println!("\n(a?){{n}} a{{n}} against n a's:");
    for n in (1000..=10000u32).step_by(1000) {
        let s = "a".repeat(n as usize);
        let start = Instant::now();
        let result = matcher(evil1(n), &s);
        println!(
            "n = {n:5}: {result} in {:.6}s",
            start.elapsed().as_secs_f64()
        );
    }

    println!("\n(a*)* b against n a's:");
    for n in (1000..=10000).step_by(1000) {
        let s = "a".repeat(n);
        let start = Instant::now();
        let result = matcher(evil2(), &s);
        println!(
            "n = {n:5}: {result} in {:.6}s",
            start.elapsed().as_secs_f64()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evil1_matches_between_n_and_2n_as() {
        for n in (100..=1000u32).step_by(100) {
            let len = n as usize;
            assert!(matcher(evil1(n), &"a".repeat(len)), "n = {n}, {len} a's");
            assert!(
                matcher(evil1(n), &"a".repeat(2 * len)),
                "n = {n}, {} a's",
                2 * len
            );
            assert!(
                !matcher(evil1(n), &"a".repeat(len - 1)),
                "n = {n}, {} a's",
                len - 1
            );
            assert!(
                !matcher(evil1(n), &"a".repeat(2 * len + 1)),
                "n = {n}, {} a's",
                2 * len + 1
            );
        }
    }

    #[test]
    fn evil2_rejects_only_as() {
        for n in (1000..=10000).step_by(1000) {
            assert!(!matcher(evil2(), &"a".repeat(n)), "n = {n}");
        }
    }

    #[test]
    fn evil2_accepts_as_then_b() {
        for n in (1000..=10000).step_by(1000) {
            let s = format!("{}b", "a".repeat(n));
            assert!(matcher(evil2(), &s), "n = {n}");
        }
    }
}
