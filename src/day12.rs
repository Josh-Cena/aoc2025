use std::collections::HashSet;
use std::fmt::Debug;

struct Shape {
    cells: Vec<(usize, usize)>,
    block_index: usize,
}

impl Debug for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut max_i = 0;
        let mut max_j = 0;
        for &(i, j) in &self.cells {
            if i > max_i {
                max_i = i;
            }
            if j > max_j {
                max_j = j;
            }
        }
        let mut grid = vec![vec![' '; max_j + 1]; max_i + 1];
        for (i, j) in &self.cells {
            grid[*i][*j] = '#';
        }
        for row in grid {
            writeln!(f, "{}", row.iter().collect::<String>())?;
        }
        Ok(())
    }
}

fn parse_input(data: Vec<String>) -> (Vec<Shape>, Vec<(usize, usize, Vec<usize>)>) {
    let input = data.join("\n");
    let chunks = input.split("\n\n").collect::<Vec<&str>>();
    let shapes = chunks[0..chunks.len() - 1]
        .iter()
        .map(|&x| {
            x.split("\n")
                .skip(1)
                .enumerate()
                .flat_map(|(i, row)| {
                    row.chars()
                        .enumerate()
                        .map(move |(j, c)| (i, j, c))
                        .filter_map(|(i, j, c)| if c == '#' { Some((i, j)) } else { None })
                })
                .collect::<Vec<_>>()
        })
        .enumerate()
        .flat_map(|(i, cells)| {
            let mut attempted_shapes = HashSet::new();
            for orientation in 0..12 {
                let mut new_block = cells.clone();
                new_block = match orientation % 4 {
                    // Clockwise 90: (0, 0) -> (0, 2), (0, 1) -> (1, 2), (1, 0) -> (0, 1)
                    1 => new_block.iter().map(|&(i, j)| (j, 2 - i)).collect(),
                    // Clockwise 180: (0, 0) -> (2, 2), (0, 1) -> (2, 1), (1, 0) -> (1, 2)
                    2 => new_block.iter().map(|&(i, j)| (2 - i, 2 - j)).collect(),
                    // Clockwise 270: (0, 0) -> (2, 0), (0, 1) -> (1, 0), (1, 0) -> (2, 1)
                    3 => new_block.iter().map(|&(i, j)| (2 - j, i)).collect(),
                    _ => new_block,
                };
                new_block = match orientation / 4 {
                    // Horizontal: (0, 0) -> (0, 2), (0, 1) -> (0, 1), (1, 0) -> (1, 2)
                    1 => new_block.iter().map(|&(i, j)| (i, 2 - j)).collect(),
                    // Vertical: (0, 0) -> (2, 0), (0, 1) -> (1, 0), (1, 0) -> (2, 1)
                    2 => new_block.iter().map(|&(i, j)| (2 - i, j)).collect(),
                    _ => new_block,
                };
                new_block.sort();
                attempted_shapes.insert(new_block.clone());
            }
            attempted_shapes
                .into_iter()
                .map(|cells| Shape {
                    cells,
                    block_index: i,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let regions = chunks[chunks.len() - 1]
        .split("\n")
        .map(|x| {
            let parts = x.split(": ").collect::<Vec<_>>();
            let size = parts[0]
                .split('x')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let counts = parts[1]
                .split(' ')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (size[0], size[1], counts)
        })
        .collect::<Vec<_>>();
    (shapes, regions)
}

fn drop_shape(shape: &Shape, board: &Vec<Vec<bool>>, col: usize) -> Option<Vec<Vec<bool>>> {
    'search_row: for i in 0..board.len() - 2 {
        for (di, dj) in &shape.cells {
            if board[i + *di][col + *dj] {
                continue 'search_row;
            }
        }
        let mut new_board = board.clone();
        for (di, dj) in &shape.cells {
            new_board[i + *di][col + *dj] = true;
        }
        return Some(new_board);
    }
    None
}

// It's like a Tetris game. For each shape, we can drop it from the top of the board
// and from any column (search all possible columns).
fn search(shapes: &Vec<Shape>, board: &Vec<Vec<bool>>, counts: &Vec<usize>) -> bool {
    if counts.iter().sum::<usize>() == 0 {
        return true;
    }
    for shape in shapes {
        if counts[shape.block_index] == 0 {
            continue;
        }
        for col in 0..board[0].len() - 2 {
            if let Some(new_board) = drop_shape(shape, &board, col) {
                let mut new_counts = counts.clone();
                new_counts[shape.block_index] -= 1;
                if search(shapes, &new_board, &new_counts) {
                    return true;
                }
            }
        }
    }
    false
}

pub fn solve1(data: Vec<String>) {
    let (shapes, regions) = parse_input(data);
    let mut total = 0;
    for (width, height, counts) in regions {
        let definitely_fits = (width / 3) * (height / 3) >= counts.iter().sum::<usize>();
        let definitely_not_fit = width * height
            < counts
                .iter()
                .enumerate()
                .map(|(i, &x)| {
                    x * shapes
                        .iter()
                        .find(|s| s.block_index == i)
                        .unwrap()
                        .cells
                        .len()
                })
                .sum::<usize>();
        if definitely_fits {
            total += 1;
        } else if !definitely_not_fit {
            // Bro you can't be serious
            // This branch is never entered
            println!("Unknown: {:?} {:?} {:?}", width, height, counts);
            if search(&shapes, &vec![vec![false; width]; height], &counts) {
                total += 1;
            }
        }
    }
    println!("{}", total);
}

pub fn solve2(_: Vec<String>) {
    println!("No such thing, yay");
}
