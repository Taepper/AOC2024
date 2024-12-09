use tae_aoclib2025::solve_all_inputs;

fn main() {
    solve_all_inputs("day_09", do_task)
}

fn do_task(input: &String) -> (i64, i64) {
    let debug_print =
        std::env::var("DEBUG_PRINT").unwrap_or("0".to_string()) == "1" && input.len() < 1000;

    let (disk, files_index, free_space_index) = parse_input(input);

    if debug_print {
        println!("{:?}", disk);
    }

    let fragmented_disk = compact_disk(&disk);
    let result1 = disk_hash(&fragmented_disk);
    if debug_print {
        println!("{:?}", fragmented_disk);
    }

    let defragmented_disk = defrag(disk, files_index, free_space_index);
    let result2 = disk_hash(&defragmented_disk);
    if debug_print {
        println!("{:?}", defragmented_disk);
    }

    (result1 as i64, result2 as i64)
}

fn disk_hash(disk: &Vec<usize>) -> usize {
    let mut hash = 0;
    for (pos, id) in disk.iter().enumerate() {
        if *id == usize::MAX {
            continue;
        }
        hash += id * pos;
    }
    hash
}

fn compact_disk(disk: &Vec<usize>) -> Vec<usize> {
    let mut fragmented_disk = disk.clone();
    let mut free_cursor = 0;
    let mut end_cursor = fragmented_disk.len() - 1;
    while free_cursor < end_cursor {
        if fragmented_disk[free_cursor] != usize::MAX {
            free_cursor += 1;
            continue;
        }
        if fragmented_disk[end_cursor] == usize::MAX {
            end_cursor -= 1;
            continue;
        }
        fragmented_disk[free_cursor] = fragmented_disk[end_cursor];
        fragmented_disk[end_cursor] = usize::MAX;
    }
    fragmented_disk
}

fn defrag(
    disk: Vec<usize>,
    files_index: Vec<(usize, usize)>,
    mut free_space_index: Vec<(usize, usize)>,
) -> Vec<usize> {
    let mut defragmented_disk = disk.clone();
    for (file_size, idx_file) in files_index.iter().rev() {
        let file_id = disk[*idx_file];
        for (free_space_size, idx_free) in free_space_index.iter_mut() {
            if *idx_free > *idx_file {
                break;
            }
            if *file_size <= *free_space_size {
                for i in 0..*file_size {
                    defragmented_disk[*idx_free + i] = file_id;
                    defragmented_disk[*idx_file + i] = usize::MAX;
                }
                *free_space_size -= *file_size;
                *idx_free += *file_size;
                break;
            }
        }
    }
    defragmented_disk
}

fn parse_input(input: &String) -> (Vec<usize>, Vec<(usize, usize)>, Vec<(usize, usize)>) {
    let char_vec = input.chars().collect::<Vec<char>>();

    let mut disk = Vec::new();
    let mut files_index = Vec::new();
    let mut free_space_index = Vec::new();

    for (id, x) in char_vec.chunks(2).enumerate() {
        assert!(x.len() <= 2);
        if x.len() == 2 {
            let (files, empty) = (x[0] as usize - '0' as usize, x[1] as usize - '0' as usize);
            files_index.push((files, disk.len()));
            for _x in 0..files {
                disk.push(id);
            }
            free_space_index.push((empty, disk.len()));
            for _x in 0..empty {
                disk.push(usize::MAX);
            }
        } else {
            assert_eq!(x.len(), 1);
            let files = x[0] as usize - '0' as usize;
            files_index.push((files, disk.len()));
            for _x in 0..files {
                disk.push(id);
            }
        }
    }
    (disk, files_index, free_space_index)
}
