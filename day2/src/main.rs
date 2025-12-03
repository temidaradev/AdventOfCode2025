use std::fs;

fn is_repeated_sequence(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();

    if len % 2 != 0 {
        return false;
    }

    let half = len / 2;
    &s[..half] == &s[half..]
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let input_vec: Vec<&str> = file.trim().split(',').collect();
    let mut answer: u64 = 0;

    for input in input_vec {
        let range: Vec<u64> = input
            .split('-')
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        for i in range[0]..=range[1] {
            if is_repeated_sequence(i) {
                answer += i;
            }
        }
    }

    println!("answer: {}", answer);
}
