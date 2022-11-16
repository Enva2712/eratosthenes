use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Default)]
pub struct Siev {
    head: u64,
    // a binary heap probably isn't the best data structure here
    wires: BinaryHeap<Wire>,
}

impl Siev {
    pub fn new() -> Self {
        Default::default()
    }

    /// zero-indexed
    pub fn nth(n: usize) -> u64 {
        Siev::new()
            .nth(n)
            .expect("Siev Iterator terminated. it never should")
    }

    pub fn next_prime(&mut self) -> u64 {
        let next_prime = loop {
            let next_wire = self.wires.peek_mut();
            match next_wire {
                None => {
                    self.head = 2; // lets us derive Default
                    break self.head;
                }
                Some(mut v) => match v.val.cmp(&self.head) {
                    Ordering::Less => {
                        // if the next siev wire is lower than the head, advance towards it
                        v.val += v.prm;
                    }
                    Ordering::Equal => {
                        // if we land on it exactly, it's nonprime
                        self.head += 1;
                    }
                    Ordering::Greater => {
                        // but if all wires have passed, it's prime
                        let prm = self.head;
                        drop(v);
                        self.head += 1;
                        break prm;
                    }
                },
            }
        };
        self.wires.push(Wire {
            val: next_prime,
            prm: next_prime,
        });
        next_prime
    }
}

impl Iterator for Siev {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        Some(self.next_prime())
    }
}

/// because what else would you make a siev out of?
#[derive(Debug, PartialEq, Eq)]
struct Wire {
    val: u64,
    prm: u64,
}

/// ordered by reverse value so peeking heap yields lowest multiple
impl PartialOrd for Wire {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.val.partial_cmp(&self.val)
    }
}

/// ordered by reverse value so peeking heap yeilds lowest multiple
impl Ord for Wire {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nth() {
        assert_eq!(Siev::nth(0), 2);
        assert_eq!(Siev::nth(49), 229);
    }

    #[test]
    fn wire_ord() {
        let wire3 = Wire { val: 3, prm: 3 };
        let wire8 = Wire { val: 8, prm: 2 };
        assert!(wire3 > wire8);
    }
}
