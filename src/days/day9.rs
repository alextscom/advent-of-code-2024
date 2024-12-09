pub fn run() -> String {
    let input = include_str!("../../inputs/day9.txt");
    solve(input)
}

fn solve(input: &str) -> String {
    let data: Vec<usize> = input.trim().chars().map(|digit_chars| digit_chars.to_digit(10).unwrap() as usize).collect();
    let mut blocks: Vec<usize> = vec![0; data.iter().sum::<usize>()];
    let mut spare = Vec::new();
    let mut add: usize = 0;
    let mut files = Vec::new();

    for (index, &size) in data.iter().enumerate() {
        if index % 2 == 1 {
            if size > 0 {
                spare.extend(add..add + size);
            }
        } else {
            let file_id = (index / 2) as usize;
            for i in add..add + size {
                blocks[i] = file_id;
                files.push((file_id, add, size));
            }
        }
        add += size;
    }

    let mut read_pointer = blocks.len() - 1;
    let mut write_pointer = spare.remove(0);
    while read_pointer > write_pointer {
        blocks[write_pointer] = blocks[read_pointer];
        blocks[read_pointer] = 0;

        write_pointer = spare.remove(0);
        read_pointer -= 1;
        while read_pointer > write_pointer && blocks[read_pointer] == 0 {
            read_pointer -= 1;
        }
    }

    let result: usize = get_result(blocks);

    // move to own function to keep part1
    part2(input);
    result.to_string()
}

fn get_result(blocks: Vec<usize>) -> usize {
    blocks.iter().enumerate().map(|(index, &val)| index as usize * val).sum()
}

fn part2(input: &str) -> usize {
    let data: Vec<usize> = input.trim().chars().map(|digit_chars| digit_chars.to_digit(10).unwrap() as usize).collect();
    let mut blocks: Vec<usize> = vec![0; data.iter().sum::<usize>()];
    let mut files = Vec::new();
    let mut spare = Vec::new();
    let mut add: usize = 0usize;

    for (index, &size) in data.iter().enumerate() {
        if index % 2 == 1 {
            if size > 0 {
                spare.push((add, size));
            }
        } else {
            let file_id = (index / 2) as usize;
            files.push((file_id, add, size));
        }
        add += size;
    }

    let mut file_pointer = files.len() as isize - 1;
    while file_pointer > 0 {
        let (file_id, add, size) = files[file_pointer as usize];
        let mut relocate = false;
        let mut index = 0;

        for (i, &(start, length)) in spare.iter().enumerate() {
            if length >= size && start < add {
                relocate = true;
                index = i;
                break;
            }
        }

        if relocate {
            let (start, length) = spare[index];
            files[file_pointer as usize] = (file_id, start, size);
            if size == length {
                spare.remove(index);
            } else {
                spare[index] = (start + size, length - size);
            }
        }
        file_pointer -= 1;
    }

    for &(file_id, add, size) in &files {
        for i in add..add + size {
            blocks[i] = file_id;
        }
    }

    let result: usize = get_result(blocks);
    println!("Part 2: {}", result);
    result
}
