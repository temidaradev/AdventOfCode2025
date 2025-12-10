use rustc_hash::FxHashSet;

pub fn part1(input: &str) -> String {
    let mut line_iter = input.lines().enumerate();
    let s_position = line_iter
        .next()
        .unwrap()
        .1
        .chars()
        .position(|val| val == 'S')
        .unwrap();
    let (_, set): (FxHashSet<usize>, FxHashSet<(usize, usize)>) = line_iter.fold(
        (
            {
                let mut set = FxHashSet::default();
                set.insert(s_position);
                set
            },
            FxHashSet::default(),
        ),
        |(positions, mut splitters), (y_index, line)| {
            let mut new_positions = FxHashSet::<usize>::default();
            for index in positions {
                if line.as_bytes()[index] == b'^' as u8 {
                    new_positions.insert(index - 1);
                    new_positions.insert(index + 1);
                    splitters.insert((index, y_index));
                } else {
                    new_positions.insert(index);
                }
            }

            (new_positions, splitters)
        },
    );
    set.len().to_string()
}

pub fn part2(input: &str) -> String {
    todo!()
}
