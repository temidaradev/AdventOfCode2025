use std::fs;

fn get_max(input: &str) -> u64 {
    let mut total = 0;

    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();
        let mut best = 0u64;

        for i in 0..chars.len() {
            for j in (i + 1)..chars.len() {
                let n = (chars[i] as u8 - b'0') as u64 * 10 + (chars[j] as u8 - b'0') as u64;
                if n > best {
                    best = n;
                }
            }
        }

        total += best;
    }

    total
}

fn main() {
    let raw = fs::read_to_string("input.txt").unwrap();
    let out = get_max(&raw);
    println!("{out}");
}
