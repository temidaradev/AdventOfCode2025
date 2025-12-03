use std::fs;

fn get_divisors(n: u32) -> Vec<u32> {
    let mut v = Vec::new();
    let n_sqrt = (n as f32).sqrt() as u32 + 1;

    for i in 1..n_sqrt {
        if n % i == 0 {
            if n / i == i {
                v.push(i);
            } else {
                v.push(i);
                v.push(n / i);
            }
        }
    }
    v.sort();
    v
}

fn main() {
    let mut answer = 0;
    let file = fs::read_to_string("input.txt").unwrap();
    let input_vec: Vec<&str> = file.split(",").map(|s| s.trim_end_matches('\n')).collect();
    println!("{:?}", input_vec.clone());
    for input in input_vec {
        let ranges = input
            .splitn(2, "-")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        'outer: for i in ranges[0]..=ranges[1] {
            let digits = i.ilog10() + 1;
            let mut chunk_sizes = get_divisors(digits);
            chunk_sizes.reverse();
            'inner: for chunk_size in chunk_sizes {
                if chunk_size == digits {
                    continue 'inner;
                }
                let chars = i.to_string().chars().collect::<Vec<char>>();
                let mut chunks = chars.chunks(chunk_size as usize).collect::<Vec<&[char]>>();
                chunks.sort_unstable();
                if chunks[0] == chunks[chunks.len() - 1] {
                    println!("found: {}", i);
                    answer += i;
                    continue 'outer;
                }
            }
        }
    }
    println!("answer: {}", answer);
}
