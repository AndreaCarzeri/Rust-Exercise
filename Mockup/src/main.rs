struct BinIter {
    n: u128, // Store the number
    length: usize, // Store the total number of bits
    current: usize, // Track the current bit index
}

impl BinIter {
    // Create a new BinIter
    fn new(n: u128, l: usize) -> Self {
        Self {
            n,
            length: l,
            current: 0,
        }
    }
}

impl Iterator for BinIter {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        // If we've iterated through all bits, stop
        if self.current >= self.length {
            return None;
        }

        // Extract the current bit
        let bit = (self.n & (1 << self.current)) != 0;

        // Move to the next bit
        self.current += 1;

        // Return the current bit
        Some(bit)
    }
}

fn main() {
    // Example 1
    for n in BinIter::new(4641312598359562305508665788689819792, 128) {
        print!("{}", if n { 1 } else { 0 });
    }
    println!();

    // Example 2
    for n in BinIter::new(18956403638425120546, 64) {
        print!("{}", if n { 1 } else { 0 });
    }
    println!();

    // Example 3
    for n in BinIter::new(15, 4) {
        print!("{}", if n { 1 } else { 0 });
    }
    println!();
}
