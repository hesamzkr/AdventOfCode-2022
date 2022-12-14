use std::collections::{HashMap, VecDeque};

#[derive(PartialEq, Clone, Debug)]
enum Tile {
    Ground,
    Elf,
}

pub fn run(input: String) -> (u32, u32) {
    let lines = input.lines().enumerate();
    let mut map: HashMap<(i32, i32), Tile> = HashMap::new();

    for (y, line) in lines {
        for (x, c) in line.chars().enumerate() {
            map.insert((x as i32, y as i32), match c {
                '.' => Tile::Ground,
                '#' => Tile::Elf,
                _ => panic!("Invalid tile character"),
            });
        }
    }

    let answer_one = part_one(map.clone());
    let answer_two = part_two(map);

    (answer_one, answer_two)
}

fn part_one(mut map: HashMap<(i32, i32), Tile>) -> u32 {
    // north direction is -1 due to the parsing of the file starting from the top
    let mut direction_order: VecDeque<[(i32, i32); 3]> = VecDeque::from_iter([
        [(0, -1), (1, -1), (-1, -1)],
        [(0, 1), (1, 1), (-1, 1)],
        [(-1, 0), (-1, -1), (-1, 1)],
        [(1, 0), (1, -1), (1, 1)],
    ]);

    for _ in 0..10 {
        let mut movements: Vec<((i32, i32), (i32, i32))> = vec![];
        'elf: for ((x, y), tile) in map.iter() {
            if tile == &Tile::Ground {
                continue;
            }

            let mut is_elves_around = false;
            for dirs in &direction_order {
                for dir in dirs {
                    if let Some(adjacent_tile) = map.get(&(x + dir.0, y + dir.1)) {
                        if adjacent_tile == &Tile::Elf {
                            is_elves_around = true;
                        }
                    }
                }
            }

            if !is_elves_around {
                continue;
            }

            'direction: for dirs in &direction_order {
                for dir in dirs {
                    if let Some(adjacent_tile) = map.get(&(x + dir.0, y + dir.1)) {
                        if adjacent_tile == &Tile::Elf {
                            continue 'direction;
                        }
                    }
                }

                movements.push(((*x, *y), (x + dirs[0].0, y + dirs[0].1)));
                continue 'elf;
            }
        }

        while !movements.is_empty() {
            let (current, proposed) = movements.pop().unwrap();
            if movements.iter().map(|x| x.1).any(|x| x == proposed) {
                movements.retain(|&item| item.1 != proposed);
            } else {
                map.insert(current, Tile::Ground);
                map.insert(proposed, Tile::Elf);
            }
        }

        let dir = direction_order.pop_front().unwrap();
        direction_order.push_back(dir);
    }

    let (min_x, max_x, min_y, max_y) = find_rectangle(&map);
    let mut count = 0;

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if let Some(tile) = map.get(&(x, y)) {
                if tile == &Tile::Ground {
                    count += 1;
                }
            } else {
                count += 1;
            }
        }
    }


    count
}

fn part_two(mut map: HashMap<(i32, i32), Tile>) -> u32 {
    // north direction is -1 due to the parsing of the file starting from the top
    let mut direction_order: VecDeque<[(i32, i32); 3]> = VecDeque::from_iter([
        [(0, -1), (1, -1), (-1, -1)],
        [(0, 1), (1, 1), (-1, 1)],
        [(-1, 0), (-1, -1), (-1, 1)],
        [(1, 0), (1, -1), (1, 1)],
    ]);
    let mut round = 0;
    let mut elves_moved = true;
    while elves_moved {
        round += 1;
        elves_moved = false;
        let mut movements: Vec<((i32, i32), (i32, i32))> = vec![];
        'elf: for ((x, y), tile) in map.iter() {
            if tile == &Tile::Ground {
                continue;
            }

            let mut is_elves_around = false;
            for dirs in &direction_order {
                for dir in dirs {
                    if let Some(adjacent_tile) = map.get(&(x + dir.0, y + dir.1)) {
                        if adjacent_tile == &Tile::Elf {
                            is_elves_around = true;
                        }
                    }
                }
            }

            if !is_elves_around {
                continue;
            }

            'direction: for dirs in &direction_order {
                for dir in dirs {
                    if let Some(adjacent_tile) = map.get(&(x + dir.0, y + dir.1)) {
                        if adjacent_tile == &Tile::Elf {
                            continue 'direction;
                        }
                    }
                }

                movements.push(((*x, *y), (x + dirs[0].0, y + dirs[0].1)));
                continue 'elf;
            }
        }

        while !movements.is_empty() {
            let (current, proposed) = movements.pop().unwrap();
            if movements.iter().map(|x| x.1).any(|x| x == proposed) {
                movements.retain(|&item| item.1 != proposed);
            } else {
                map.insert(current, Tile::Ground);
                map.insert(proposed, Tile::Elf);
                elves_moved = true;
            }
        }

        let dir = direction_order.pop_front().unwrap();
        direction_order.push_back(dir);


    }

    round
}

fn find_rectangle(map: &HashMap<(i32, i32), Tile>) -> (i32, i32, i32, i32) {

    let min_x = map.iter().filter(|x| x.1 == &Tile::Elf).map(|x| x.0.0).min().unwrap();
    let max_x = map.iter().filter(|x| x.1 == &Tile::Elf).map(|x| x.0.0).max().unwrap();
    let min_y = map.iter().filter(|x| x.1 == &Tile::Elf).map(|x| x.0.1).min().unwrap();
    let max_y = map.iter().filter(|x| x.1 == &Tile::Elf).map(|x| x.0.1).max().unwrap();
    
    (min_x, max_x, min_y, max_y)
}