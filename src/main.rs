//! A tiny interactive calculator that adds or subtracts n numbers.
//!
//! Run with `cargo run`. At each prompt pick an operation, then enter a
//! whitespace-separated line of numbers:
//!
//! ```text
//! op? add
//! numbers? 10 5 3
//! = 18
//! op? quit
//! ```

use std::io::{self, Write};

/// The operations the calculator understands.
enum Op {
    Add,
    Sub,
    Quit,
}

fn main() {
    println!("calc — add/subtract n numbers. Type 'quit' to exit.");

    loop {
        let op = match read_line("op? ") {
            // EOF (Ctrl+Z / Ctrl+D) or an explicit quit ends the session.
            None => break,
            Some(line) => match parse_op(&line) {
                Some(Op::Quit) => break,
                Some(op) => op,
                None => {
                    if !line.trim().is_empty() {
                        println!("  unknown operation '{}' — try: add, sub, quit", line.trim());
                    }
                    continue;
                }
            },
        };

        let numbers = match read_line("numbers? ") {
            None => break,
            Some(line) => match parse_numbers(&line) {
                Ok(nums) if nums.is_empty() => {
                    println!("  please enter at least one number");
                    continue;
                }
                Ok(nums) => nums,
                Err(msg) => {
                    println!("  {msg}");
                    continue;
                }
            },
        };

        let result = match op {
            Op::Add => add(&numbers),
            Op::Sub => sub(&numbers),
            Op::Quit => unreachable!(),
        };
        println!("= {result}");
    }

    println!("bye!");
}

/// Add every number together. An empty slice sums to 0.
fn add(nums: &[f64]) -> f64 {
    nums.iter().sum()
}

/// Subtract left to right: the first number minus all the rest.
/// A single number returns itself; an empty slice returns 0.
fn sub(nums: &[f64]) -> f64 {
    match nums.split_first() {
        Some((first, rest)) => first - rest.iter().sum::<f64>(),
        None => 0.0,
    }
}

/// Map a line of input to an operation, ignoring surrounding whitespace and case.
fn parse_op(line: &str) -> Option<Op> {
    match line.trim().to_ascii_lowercase().as_str() {
        "add" | "+" | "addition" => Some(Op::Add),
        "sub" | "-" | "subtraction" | "subtract" => Some(Op::Sub),
        "quit" | "exit" | "q" => Some(Op::Quit),
        _ => None,
    }
}

/// Parse a whitespace-separated line into floats, reporting the first bad token.
fn parse_numbers(line: &str) -> Result<Vec<f64>, String> {
    line.split_whitespace()
        .map(|tok| {
            tok.parse::<f64>()
                .map_err(|_| format!("'{tok}' is not a valid number"))
        })
        .collect()
}

/// Print a prompt and read one line from stdin.
/// Returns `None` on EOF so callers can exit cleanly.
fn read_line(prompt: &str) -> Option<String> {
    print!("{prompt}");
    io::stdout().flush().ok()?;

    let mut buf = String::new();
    match io::stdin().read_line(&mut buf) {
        Ok(0) => None, // EOF
        Ok(_) => Some(buf),
        Err(_) => None,
    }
}

// ---------------------------------------------------------------------------
// Deliberately smelly code — a target set for the sonar-rust-enterprise
// analyzer. Each function below trips one specific rule (SXXXX noted inline).
// Nothing here is wired into the REPL, so the calculator still works exactly
// as before. To get the clean calculator back: `git checkout src/main.rs`.
// ---------------------------------------------------------------------------
#[allow(dead_code)]
mod smells {
    // S1764 — identical operands on both sides of a comparison (always false).
    pub fn is_same(n: f64) -> bool {
        n < n
    }

    // S107 — too many parameters (8 > default max of 7).
    pub fn add_many(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64, g: f64, h: f64) -> f64 {
        a + b + c + d + e + f + g + h
    }

    // S9038 — empty infinite loop (busy-wait that wastes CPU).
    pub fn wait_forever() {
        loop {}
    }

    // S9031 — disallowed placeholder name `foo`.
    pub fn gravity() -> f64 {
        let foo = 9.81;
        foo
    }

    // S7442 — unwrap() inside a branch that proves the value is None.
    pub fn force(opt: Option<f64>) -> f64 {
        if opt.is_none() {
            return opt.unwrap();
        }
        opt.unwrap()
    }

    // S1862 — duplicated condition in an if / else-if chain.
    pub fn classify(param: i32) -> &'static str {
        if param == 1 {
            "one"
        } else if param == 2 {
            "two"
        } else if param == 1 {
            "one again"
        } else {
            "many"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_sums_all() {
        assert_eq!(add(&[10.0, 5.0, 3.0]), 18.0);
    }

    #[test]
    fn add_empty_is_zero() {
        assert_eq!(add(&[]), 0.0);
    }

    #[test]
    fn add_handles_decimals() {
        assert_eq!(add(&[1.5, 2.25]), 3.75);
    }

    #[test]
    fn sub_left_to_right() {
        assert_eq!(sub(&[10.0, 5.0, 3.0]), 2.0);
    }

    #[test]
    fn sub_single_is_itself() {
        assert_eq!(sub(&[7.0]), 7.0);
    }

    #[test]
    fn sub_handles_negatives() {
        assert_eq!(sub(&[-5.0, -2.0]), -3.0);
    }

    #[test]
    fn sub_empty_is_zero() {
        assert_eq!(sub(&[]), 0.0);
    }

    #[test]
    fn parse_numbers_valid() {
        assert_eq!(parse_numbers("10 5 3").unwrap(), vec![10.0, 5.0, 3.0]);
    }

    #[test]
    fn parse_numbers_empty_line() {
        assert_eq!(parse_numbers("   ").unwrap(), Vec::<f64>::new());
    }

    #[test]
    fn parse_numbers_bad_token() {
        assert!(parse_numbers("10 oops 3").is_err());
    }
}
